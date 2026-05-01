// devela::data::codec::crypto::sha1
//
//! Defines [`Sha1`].
//
// NOTES
// - Implements SHA-1 from the public algorithm specification.
// - This is not adapted from any third-party implementation.
// - References:
//   - FIPS 180-4, Secure Hash Standard.
//   - RFC 3174, US Secure Hash Algorithm 1.

use crate::{_impl_init, CryptoError, Digest, Slice, cmp, is, unwrap, whilst};

type Sha1Digest = Digest<{ Sha1::DIGEST_LEN }>;

#[doc = crate::_tags!(crypto hash)]
/// Incremental SHA-1 state.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// SHA-1 is provided for compatibility with existing formats and protocols.
///
/// Do not use it for new security designs, digital signatures, password hashing,
/// or collision-resistant identification.
///
/// This implementation is allocation-free, compile-time friendly,
/// and supports incremental updates.
///
/// Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize]
/// to consume the state and return the 20-byte digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sha1 {
    state: [u32; 5],
    len_bits: u64,
    block: [u8; Sha1::BLOCK_LEN],
    block_len: u8,
}
_impl_init![ConstInit: Self::new() => Sha1];
impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Sha1 {
    /// The digest size in bytes.
    pub const DIGEST_LEN: usize = 20;

    /// The internal block size in bytes.
    pub const BLOCK_LEN: usize = 64;

    /// Creates a new SHA-1 state.
    pub const fn new() -> Self {
        Self {
            state: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476, 0xC3D2_E1F0],
            len_bits: 0,
            block: [0; Sha1::BLOCK_LEN],
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
    /// would exceed SHA-1's 64-bit bit-length field.
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
    /// Computes the SHA-1 digest of `bytes`.
    ///
    /// # Errors
    /// Returns [`CryptoError::LengthOverflow`] if `bytes` exceeds SHA-1's supported message length.
    pub const fn digest_bytes(bytes: &[u8]) -> Result<Sha1Digest, CryptoError> {
        let mut sha = Self::new();
        unwrap![ok? sha.update(bytes)];
        Ok(sha.finalize())
    }
    /// Finalizes the digest and consumes the state.
    ///
    /// This method cannot fail.
    pub const fn finalize(mut self) -> Sha1Digest {
        let len_bits = self.len_bits;
        self.push_padding_byte(0x80);
        while self.block_len != 56 {
            self.push_padding_byte(0);
        }
        let len = len_bits.to_be_bytes();
        whilst! { i in 0..len.len(); {
            self.push_padding_byte(len[i]);
        }}
        let mut out = [0u8; Sha1::DIGEST_LEN];
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
            let free = Sha1::BLOCK_LEN - used;
            let take = cmp![min free, bytes.len()];
            Slice::range_mut(&mut self.block, used, used + take)
                .copy_from_slice(Slice::range_to(&bytes, take));
            self.block_len += take as u8;
            bytes = Slice::range_from(&bytes, take);
            if self.block_len as usize == Sha1::BLOCK_LEN {
                let block = self.block;
                self.process_block(&block);
                self.block_len = 0;
            }
        }
        while bytes.len() >= Sha1::BLOCK_LEN {
            let mut block = [0u8; Sha1::BLOCK_LEN];
            block.copy_from_slice(Slice::range_to(&bytes, Sha1::BLOCK_LEN));
            self.process_block(&block);
            bytes = Slice::range_from(&bytes, Sha1::BLOCK_LEN);
        }
        if !bytes.is_empty() {
            Slice::range_to_mut(&mut self.block, bytes.len()).copy_from_slice(bytes);
            self.block_len = bytes.len() as u8;
        }
    }
    const fn push_padding_byte(&mut self, byte: u8) {
        self.block[self.block_len as usize] = byte;
        self.block_len += 1;
        if self.block_len as usize == Sha1::BLOCK_LEN {
            let block = self.block;
            self.process_block(&block);
            self.block = [0; Sha1::BLOCK_LEN];
            self.block_len = 0;
        }
    }
    const fn process_block(&mut self, block: &[u8; Sha1::BLOCK_LEN]) {
        let mut w = [0u32; 80];
        whilst! { t in 0..16; {
            let i = t * 4;
            w[t] = u32::from_be_bytes([block[i], block[i + 1], block[i + 2], block[i + 3]]);
        }}
        whilst! { t in 16..80; {
            w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1);
        }}
        let [mut a, mut b, mut c, mut d, mut e] = self.state;
        whilst! { t in 0..80; {
            let (f, k) = if t < 20 {
                ((b & c) | ((!b) & d), 0x5A82_7999)
            } else if t < 40 {
                (b ^ c ^ d, 0x6ED9_EBA1)
            } else if t < 60 {
                ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC)
            } else {
                (b ^ c ^ d, 0xCA62_C1D6)
            };
            let temp =
                a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[t]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }}
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }
}

#[cfg(test)]
mod tests {
    use super::{Digest, Sha1, Sha1Digest, whilst};

    fn hex_value(byte: u8) -> u8 {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => panic!("invalid hex digit"),
        }
    }
    fn digest_from_hex(hex: &str) -> Sha1Digest {
        let bytes = hex.as_bytes();
        assert_eq!(bytes.len(), Sha1::DIGEST_LEN * 2);
        let mut out = [0u8; Sha1::DIGEST_LEN];
        whilst! { i in 0..Sha1::DIGEST_LEN; {
            out[i] = (hex_value(bytes[i * 2]) << 4) | hex_value(bytes[i * 2 + 1]);
        }}
        Digest(out)
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Sha1::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }
    #[test]
    fn known_vectors() {
        assert_digest(b"", "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_digest(b"abc", "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_digest(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
        );
    }
    #[test]
    fn million_a() {
        let mut sha = Sha1::new();
        for _ in 0..1_000_000 {
            sha.update(b"a").unwrap();
        }
        assert_eq!(sha.finalize(), digest_from_hex("34aa973cd4c4daa4f61eeb2bdbad27316534016f"),);
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Sha1::digest_bytes(input).unwrap();
        let mut chunked = Sha1::new();
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
            let full = Sha1::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Sha1::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut sha = Sha1::new();
        sha.update(b"abc").unwrap();
        sha.reset();
        sha.update(b"abc").unwrap();
        assert_eq!(sha.finalize(), digest_from_hex("a9993e364706816aba3e25717850c26c9cd0d89d"),);
    }
}
