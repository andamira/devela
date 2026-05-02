// devela::data::codec::crypto::sha256
//
//! Defines [`Sha256`].
//
// NOTES
// - Implements SHA-256 from the public algorithm specification.
// - This is not adapted from any third-party implementation.
// - References:
//   - FIPS 180-4, Secure Hash Standard.
//   - RFC 6234, US Secure Hash Algorithms.
//   - RFC 2104, Keyed-Hash Message Authentication Code.
//   - RFC 4231, Identifiers and Test Vectors for HMAC-SHA-224,
//     HMAC-SHA-256, HMAC-SHA-384, and HMAC-SHA-512.

use crate::{_crypto_impl_hmac, _impl_init, CryptoError, Digest, Slice, cmp, is, unwrap, whilst};

type Sha256Digest = Digest<{ Sha256::DIGEST_LEN }>;

#[doc = crate::_tags!(crypto hash)]
/// Incremental SHA-256 state.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// SHA-256 is provided as a fixed-size cryptographic message digest.
///
/// This implementation is allocation-free, compile-time friendly,
/// and supports incremental updates.
///
/// Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize]
/// to consume the state and return the 32-byte digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sha256 {
    state: [u32; 8],
    len_bits: u64,
    block: [u8; Sha256::BLOCK_LEN],
    block_len: u8,
}
_impl_init![ConstInit: Self::new() => Sha256];
impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}
impl Sha256 {
    /// The digest size in bytes.
    pub const DIGEST_LEN: usize = 32;
    /// The internal block size in bytes.
    pub const BLOCK_LEN: usize = 64;
    #[rustfmt::skip]
    const K: [u32; 64] = [
        0x428A_2F98, 0x7137_4491, 0xB5C0_FBCF, 0xE9B5_DBA5,
        0x3956_C25B, 0x59F1_11F1, 0x923F_82A4, 0xAB1C_5ED5,
        0xD807_AA98, 0x1283_5B01, 0x2431_85BE, 0x550C_7DC3,
        0x72BE_5D74, 0x80DE_B1FE, 0x9BDC_06A7, 0xC19B_F174,
        0xE49B_69C1, 0xEFBE_4786, 0x0FC1_9DC6, 0x240C_A1CC,
        0x2DE9_2C6F, 0x4A74_84AA, 0x5CB0_A9DC, 0x76F9_88DA,
        0x983E_5152, 0xA831_C66D, 0xB003_27C8, 0xBF59_7FC7,
        0xC6E0_0BF3, 0xD5A7_9147, 0x06CA_6351, 0x1429_2967,
        0x27B7_0A85, 0x2E1B_2138, 0x4D2C_6DFC, 0x5338_0D13,
        0x650A_7354, 0x766A_0ABB, 0x81C2_C92E, 0x9272_2C85,
        0xA2BF_E8A1, 0xA81A_664B, 0xC24B_8B70, 0xC76C_51A3,
        0xD192_E819, 0xD699_0624, 0xF40E_3585, 0x106A_A070,
        0x19A4_C116, 0x1E37_6C08, 0x2748_774C, 0x34B0_BCB5,
        0x391C_0CB3, 0x4ED8_AA4A, 0x5B9C_CA4F, 0x682E_6FF3,
        0x748F_82EE, 0x78A5_636F, 0x84C8_7814, 0x8CC7_0208,
        0x90BE_FFFA, 0xA450_6CEB, 0xBEF9_A3F7, 0xC671_78F2,
    ];
    /// Creates a new SHA-256 state.
    #[rustfmt::skip]
    pub const fn new() -> Self {
        Self {
            state: [
                0x6A09_E667, 0xBB67_AE85, 0x3C6E_F372, 0xA54F_F53A,
                0x510E_527F, 0x9B05_688C, 0x1F83_D9AB, 0x5BE0_CD19,
            ],
            len_bits: 0,
            block: [0; Sha256::BLOCK_LEN],
            block_len: 0,
        }
    }
    /// Returns the number of message bits already accepted.
    pub const fn len_bits(&self) -> u64 {
        self.len_bits
    }
    /// Returns `true` if no message bytes have been accepted.
    pub const fn is_empty(&self) -> bool {
        self.len_bits == 0
    }
    /// Resets the state to its initial value.
    pub const fn reset(&mut self) {
        *self = Self::new();
    }
    /// Updates the digest state with `bytes`.
    ///
    /// # Errors
    /// Returns [`CryptoError::LengthOverflow`] if the total message length
    /// would exceed SHA-256's 64-bit bit-length field.
    pub const fn update(&mut self, bytes: &[u8]) -> Result<(), CryptoError> {
        is! { bytes.len() > (u64::MAX / 8) as usize, return Err(CryptoError::LengthOverflow) }
        let add_bits = (bytes.len() as u64) * 8;
        let Some(new_len_bits) = self.len_bits.checked_add(add_bits) else {
            return Err(CryptoError::LengthOverflow);
        };
        self.len_bits = new_len_bits;
        self.write_blocks(bytes);
        Ok(())
    }
    /// Computes the SHA-256 digest of `bytes`.
    ///
    /// # Errors
    /// Returns [`CryptoError::LengthOverflow`] if `bytes` exceeds SHA-256's
    /// supported message length.
    pub const fn digest_bytes(bytes: &[u8]) -> Result<Sha256Digest, CryptoError> {
        let mut sha = Self::new();
        unwrap![ok? sha.update(bytes)];
        Ok(sha.finalize())
    }

    _crypto_impl_hmac![Sha256, Sha256Digest, "HMAC-SHA-256"];

    /// Finalizes the digest and consumes the state.
    ///
    /// This method cannot fail.
    pub const fn finalize(mut self) -> Sha256Digest {
        let len_bits = self.len_bits;
        self.push_padding_byte(0x80);
        while self.block_len != 56 {
            self.push_padding_byte(0);
        }
        let len = len_bits.to_be_bytes();
        whilst! { i in 0..len.len(); { self.push_padding_byte(len[i]); }}
        let mut out = [0u8; Sha256::DIGEST_LEN];
        whilst! { i in 0..self.state.len(); {
            let bytes = self.state[i].to_be_bytes();
            let j = i * 4;
            out[j] = bytes[0];
            out[j + 1] = bytes[1];
            out[j + 2] = bytes[2];
            out[j + 3] = bytes[3];
        }}
        Digest(out)
    }
    const fn write_blocks(&mut self, mut bytes: &[u8]) {
        if self.block_len != 0 {
            let used = self.block_len as usize;
            let free = Sha256::BLOCK_LEN - used;
            let take = cmp![min free, bytes.len()];
            Slice::range_mut(&mut self.block, used, used + take)
                .copy_from_slice(Slice::range_to(&bytes, take));
            self.block_len += take as u8;
            bytes = Slice::range_from(&bytes, take);
            if self.block_len as usize == Sha256::BLOCK_LEN {
                let block = self.block;
                self.process_block(&block);
                self.block_len = 0;
            }
        }
        while bytes.len() >= Sha256::BLOCK_LEN {
            let mut block = [0u8; Sha256::BLOCK_LEN];
            block.copy_from_slice(Slice::range_to(&bytes, Sha256::BLOCK_LEN));
            self.process_block(&block);
            bytes = Slice::range_from(&bytes, Sha256::BLOCK_LEN);
        }
        if !bytes.is_empty() {
            Slice::range_to_mut(&mut self.block, bytes.len()).copy_from_slice(bytes);
            self.block_len = bytes.len() as u8;
        }
    }
    const fn push_padding_byte(&mut self, byte: u8) {
        self.block[self.block_len as usize] = byte;
        self.block_len += 1;
        if self.block_len as usize == Sha256::BLOCK_LEN {
            let block = self.block;
            self.process_block(&block);
            self.block = [0; Sha256::BLOCK_LEN];
            self.block_len = 0;
        }
    }
    const fn process_block(&mut self, block: &[u8; Sha256::BLOCK_LEN]) {
        let mut w = [0u32; 64];
        whilst! { t in 0..16; {
            let i = t * 4;
            w[t] = u32::from_be_bytes([
                block[i],
                block[i + 1],
                block[i + 2],
                block[i + 3],
            ]);
        }}
        whilst! { t in 16..64; {
            w[t] = Self::small_sigma1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(Self::small_sigma0(w[t - 15]))
                .wrapping_add(w[t - 16]);
        }}
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        whilst! { t in 0..64; {
            let t1 = h
                .wrapping_add(Self::big_sigma1(e))
                .wrapping_add(Self::ch(e, f, g))
                .wrapping_add(Self::K[t])
                .wrapping_add(w[t]);
            let t2 = Self::big_sigma0(a).wrapping_add(Self::maj(a, b, c));
            h = g; g = f; f = e; e = d.wrapping_add(t1);
            d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }}
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
    const fn ch(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ ((!x) & z)
    }
    const fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }
    const fn big_sigma0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }
    const fn big_sigma1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }
    const fn small_sigma0(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
    }
    const fn small_sigma1(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::_hex, Digest, Sha256, Sha256Digest, whilst};

    fn digest_from_hex(hex: &str) -> Sha256Digest {
        Digest(self::_hex(hex))
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Sha256::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }

    #[test]
    fn known_vectors() {
        assert_digest(b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_digest(b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_digest(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        );
    }
    #[test]
    #[cfg(not(miri))] // too slow for miri
    fn million_a() {
        let mut sha = Sha256::new();
        for _ in 0..1_000_000 {
            sha.update(b"a").unwrap();
        }
        assert_eq!(
            sha.finalize(),
            digest_from_hex("cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"),
        );
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Sha256::digest_bytes(input).unwrap();
        let mut chunked = Sha256::new();
        chunked.update(b"the quick ").unwrap();
        chunked.update(b"brown fox ").unwrap();
        chunked.update(b"jumps over ").unwrap();
        chunked.update(b"the lazy dog").unwrap();
        assert_eq!(full, chunked.finalize());
    }

    #[test]
    fn boundary_lengths() {
        for len in [0, 1, 55, 56, 57, 63, 64, 65, 119, 120] {
            let mut bytes = [0u8; 120];
            whilst! { i in 0..len; { bytes[i] = i as u8; }}
            let full = Sha256::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Sha256::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut sha = Sha256::new();
        sha.update(b"abc").unwrap();
        sha.reset();
        sha.update(b"abc").unwrap();
        assert_eq!(
            sha.finalize(),
            digest_from_hex("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        );
    }
    #[test]
    fn hmac_vectors() {
        let key = [0x0b; 20];
        assert_eq!(
            Sha256::hmac(&key, b"Hi There").unwrap(),
            digest_from_hex("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"),
        );
        assert_eq!(
            Sha256::hmac(b"Jefe", b"what do ya want for nothing?").unwrap(),
            digest_from_hex("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"),
        );
    }
}
