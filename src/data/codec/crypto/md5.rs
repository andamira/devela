// devela::data::codec::crypto::md5
//
//! Defines [`Md5`].
//
// NOTES
// - Implements MD5 from the public algorithm specification.
// - This is not adapted from any third-party implementation.
// - References:
//   - RFC 1321, The MD5 Message-Digest Algorithm.
//   - RFC 6151, Updated Security Considerations for the MD5 Message-Digest
//     and the HMAC-MD5 Algorithms.
//   - RFC 2104, Keyed-Hash Message Authentication Code.
//   - RFC 2202, Test Cases for HMAC-MD5 and HMAC-SHA-1.

use crate::{__crypto_impl_hmac, _impl_init, CryptoError, Digest, Slice, cmp, is, unwrap, whilst};

type Md5Digest = Digest<{ Md5::DIGEST_LEN }>;

#[doc = crate::_tags!(crypto hash)]
/// Incremental MD5 state.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// MD5 is provided for compatibility with existing formats and protocols.
///
/// Do not use it for new security designs, digital signatures, password hashing,
/// collision-resistant identification, or integrity protection against attackers.
///
/// This implementation is allocation-free, compile-time friendly,
/// and supports incremental updates.
///
/// Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize]
/// to consume the state and return the 16-byte digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Md5 {
    state: [u32; 4],
    len_bits: u64,
    block: [u8; Md5::BLOCK_LEN],
    block_len: u8,
}
_impl_init![ConstInit: Self::new() => Md5];
impl Default for Md5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Md5 {
    /// The digest size in bytes.
    pub const DIGEST_LEN: usize = 16;
    /// The internal block size in bytes.
    pub const BLOCK_LEN: usize = 64;
    #[rustfmt::skip]
    const K: [u32; 64] = [
        0xD76A_A478, 0xE8C7_B756, 0x2420_70DB, 0xC1BD_CEEE,
        0xF57C_0FAF, 0x4787_C62A, 0xA830_4613, 0xFD46_9501,
        0x6980_98D8, 0x8B44_F7AF, 0xFFFF_5BB1, 0x895C_D7BE,
        0x6B90_1122, 0xFD98_7193, 0xA679_438E, 0x49B4_0821,
        0xF61E_2562, 0xC040_B340, 0x265E_5A51, 0xE9B6_C7AA,
        0xD62F_105D, 0x0244_1453, 0xD8A1_E681, 0xE7D3_FBC8,
        0x21E1_CDE6, 0xC337_07D6, 0xF4D5_0D87, 0x455A_14ED,
        0xA9E3_E905, 0xFCEF_A3F8, 0x676F_02D9, 0x8D2A_4C8A,
        0xFFFA_3942, 0x8771_F681, 0x6D9D_6122, 0xFDE5_380C,
        0xA4BE_EA44, 0x4BDE_CFA9, 0xF6BB_4B60, 0xBEBF_BC70,
        0x289B_7EC6, 0xEAA1_27FA, 0xD4EF_3085, 0x0488_1D05,
        0xD9D4_D039, 0xE6DB_99E5, 0x1FA2_7CF8, 0xC4AC_5665,
        0xF429_2244, 0x432A_FF97, 0xAB94_23A7, 0xFC93_A039,
        0x655B_59C3, 0x8F0C_CC92, 0xFFEF_F47D, 0x8584_5DD1,
        0x6FA8_7E4F, 0xFE2C_E6E0, 0xA301_4314, 0x4E08_11A1,
        0xF753_7E82, 0xBD3A_F235, 0x2AD7_D2BB, 0xEB86_D391,
    ];
    #[rustfmt::skip]
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    /// Creates a new MD5 state.
    pub const fn new() -> Self {
        Self {
            state: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476],
            len_bits: 0,
            block: [0; Md5::BLOCK_LEN],
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
    /// Returns [`LengthOverflow`][CryptoError::LengthOverflow]
    /// if the total message length would exceed MD5's 64-bit bit-length field.
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
    /// Computes the MD5 digest of `bytes`.
    ///
    /// # Errors
    /// Returns [`LengthOverflow`][CryptoError::LengthOverflow]
    /// if `bytes` exceeds MD5's supported message length.
    pub const fn digest_bytes(bytes: &[u8]) -> Result<Md5Digest, CryptoError> {
        let mut md5 = Self::new();
        unwrap![ok? md5.update(bytes)];
        Ok(md5.finalize())
    }

    __crypto_impl_hmac![Md5, Md5Digest];

    /// Finalizes the digest and consumes the state.
    ///
    /// This method cannot fail.
    pub const fn finalize(mut self) -> Md5Digest {
        let len_bits = self.len_bits;
        self.push_padding_byte(0x80);
        while self.block_len != 56 {
            self.push_padding_byte(0);
        }
        let len = len_bits.to_le_bytes();
        whilst! { i in 0..len.len(); {
            self.push_padding_byte(len[i]);
        }}
        let mut out = [0u8; Md5::DIGEST_LEN];
        whilst! { i in 0..self.state.len(); {
            let bytes = self.state[i].to_le_bytes();
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
            let free = Md5::BLOCK_LEN - used;
            let take = cmp![min free, bytes.len()];
            Slice::range_mut(&mut self.block, used, used + take)
                .copy_from_slice(Slice::range_to(&bytes, take));
            self.block_len += take as u8;
            bytes = Slice::range_from(&bytes, take);
            if self.block_len as usize == Md5::BLOCK_LEN {
                let block = self.block;
                self.process_block(&block);
                self.block_len = 0;
            }
        }
        while bytes.len() >= Md5::BLOCK_LEN {
            let mut block = [0u8; Md5::BLOCK_LEN];
            block.copy_from_slice(Slice::range_to(&bytes, Md5::BLOCK_LEN));
            self.process_block(&block);
            bytes = Slice::range_from(&bytes, Md5::BLOCK_LEN);
        }
        if !bytes.is_empty() {
            Slice::range_to_mut(&mut self.block, bytes.len()).copy_from_slice(bytes);
            self.block_len = bytes.len() as u8;
        }
    }
    const fn push_padding_byte(&mut self, byte: u8) {
        self.block[self.block_len as usize] = byte;
        self.block_len += 1;
        if self.block_len as usize == Md5::BLOCK_LEN {
            let block = self.block;
            self.process_block(&block);
            self.block = [0; Md5::BLOCK_LEN];
            self.block_len = 0;
        }
    }
    const fn process_block(&mut self, block: &[u8; Md5::BLOCK_LEN]) {
        let mut m = [0u32; 16];
        whilst! { i in 0..16; {
            let j = i * 4;
            m[i] = u32::from_le_bytes([
                block[j],
                block[j + 1],
                block[j + 2],
                block[j + 3],
            ]);
        }}
        let [mut a, mut b, mut c, mut d] = self.state;
        whilst! { i in 0..64; {
            let (f, g) = if i < 16 {
                ((b & c) | ((!b) & d), i)
            } else if i < 32 {
                ((d & b) | ((!d) & c), (5 * i + 1) % 16)
            } else if i < 48 {
                (b ^ c ^ d, (3 * i + 5) % 16)
            } else {
                (c ^ (b | (!d)), (7 * i) % 16)
            };
            let next = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(Self::K[i])
                    .wrapping_add(m[g])
                    .rotate_left(Self::S[i]),
            );
            a = d;
            d = c;
            c = b;
            b = next;
        }}
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }
}

#[cfg(test)]
mod tests {
    use super::{super::_hex, Digest, Md5, Md5Digest, whilst};

    fn digest_from_hex(hex: &str) -> Md5Digest {
        Digest(self::_hex(hex))
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Md5::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc1321#appendix-A.5
    fn known_vectors() {
        assert_digest(b"", "d41d8cd98f00b204e9800998ecf8427e");
        assert_digest(b"a", "0cc175b9c0f1b6a831c399e269772661");
        assert_digest(b"abc", "900150983cd24fb0d6963f7d28e17f72");
        assert_digest(b"message digest", "f96b697d7cb7938d525a2f31aaf161d0");
        assert_digest(b"abcdefghijklmnopqrstuvwxyz", "c3fcd3d76192e4007dfb496cca67e13b");
        assert_digest(
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        );
        assert_digest(
            b"12345678901234567890123456789012345678901234567890\
              123456789012345678901234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        );
    }
    #[test]
    #[cfg(not(miri))] // too slow for miri
    fn million_a() {
        let mut md5 = Md5::new();
        for _ in 0..1_000_000 {
            md5.update(b"a").unwrap();
        }
        assert_eq!(md5.finalize(), digest_from_hex("7707d6ae4e027c70eea2a935c2296f21"),);
    }
    #[test]
    // https://www.rfc-editor.org/rfc/rfc2202.html#section-2
    fn hmac_vectors() {
        let key = [0x0b; 16];
        assert_eq!(
            Md5::hmac(&key, b"Hi There").unwrap(),
            digest_from_hex("9294727a3638bb1c13f48ef8158bfc9d"),
        );
        assert_eq!(
            Md5::hmac(b"Jefe", b"what do ya want for nothing?").unwrap(),
            digest_from_hex("750c783e6ab0b503eaa86e310a5db738"),
        );
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Md5::digest_bytes(input).unwrap();
        let mut chunked = Md5::new();
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
            let full = Md5::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Md5::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut md5 = Md5::new();
        md5.update(b"abc").unwrap();
        md5.reset();
        md5.update(b"abc").unwrap();
        assert_eq!(md5.finalize(), digest_from_hex("900150983cd24fb0d6963f7d28e17f72"),);
    }
}
