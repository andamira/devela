// devela::data::codec::crypto::sha1
//
//! Defines legacy SHA1 secure hash algorithm.
//

#[doc = crate::_tags!(crypto hash)]
/// Implements SHA-1 digest methods for a concrete hash type.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// SHA-1 is provided for compatibility with existing formats and protocols.
///
/// Do not use it for new security designs, digital signatures, password hashing,
/// or collision-resistant identification.
//
// References:
// - FIPS 180-4, Secure Hash Standard.
// - RFC 6234, US Secure Hash Algorithms.
// - RFC 2104, Keyed-Hash Message Authentication Code.
// - RFC 2202, Test Cases for HMAC-MD5 and HMAC-SHA-1.
#[doc(hidden)]
#[macro_export]
macro_rules! __crypto_impl_sha1 {
    ($(#[$attr:meta])* $vis:vis struct $Self:ident) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis struct $Self {
            state: [u32; 5],
            len_bits: u64,
            block: [u8; $Self::BLOCK_LEN],
            block_len: u8,
        }

        impl $crate::ConstInit for $Self { const INIT: $Self = $Self::new(); }
        impl Default for $Self { fn default() -> $Self { $Self::new() } }

        impl $Self {
            /// The digest size in bytes.
            pub const DIGEST_LEN: usize = 20;

            /// The internal block size in bytes.
            pub const BLOCK_LEN: usize = 64;

            /// Creates a new SHA-1 state.
            pub const fn new() -> $Self {
                $Self {
                    state: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476, 0xC3D2_E1F0],
                    len_bits: 0,
                    block: [0; $Self::BLOCK_LEN],
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
                *self = $Self::new();
            }
            /// Updates the digest state with `bytes`.
            ///
            /// # Errors
            /// Returns [`LengthOverflow`][crate::CryptoError::LengthOverflow]
            /// if the total message length would exceed SHA-1's 64-bit bit-length field.
            pub const fn update(&mut self, bytes: &[u8]) -> Result<(), $crate::CryptoError> {
                if bytes.len() > (u64::MAX / 8) as usize {
                    return Err($crate::CryptoError::LengthOverflow)
                }
                let add_bits = (bytes.len() as u64) * 8;
                let Some(new_len_bits) = self.len_bits.checked_add(add_bits) else {
                    return Err($crate::CryptoError::LengthOverflow);
                };
                self.len_bits = new_len_bits;
                self.write_blocks(bytes);
                Ok(())
            }
            /// Computes the SHA-1 digest of `bytes`.
            ///
            /// # Errors
            /// Returns [`LengthOverflow`][crate::CryptoError::LengthOverflow]
            /// if `bytes` exceeds SHA-1's supported message length.
            pub const fn digest_bytes(bytes: &[u8])
                -> Result<$crate::Digest<{ $Self::DIGEST_LEN }>, $crate::CryptoError> {
                let mut sha = $Self::new();
                $crate::unwrap![ok? sha.update(bytes)];
                Ok(sha.finalize())
            }

            $crate::__crypto_impl_hmac![$Self];

            /// Finalizes the digest and consumes the state.
            ///
            /// This method cannot fail.
            pub const fn finalize(mut self) -> $crate::Digest<{ $Self::DIGEST_LEN }> {
                let len_bits = self.len_bits;
                self.push_padding_byte(0x80);
                while self.block_len != 56 {
                    self.push_padding_byte(0);
                }
                let len = len_bits.to_be_bytes();
                $crate::whilst! { i in 0..len.len(); {
                    self.push_padding_byte(len[i]);
                }}
                let mut out = [0u8; $Self::DIGEST_LEN];
                $crate::whilst! { i in 0..self.state.len(); {
                    let bytes = self.state[i].to_be_bytes();
                    let j = i * 4;
                    out[j] = bytes[0];
                    out[j + 1] = bytes[1];
                    out[j + 2] = bytes[2];
                    out[j + 3] = bytes[3];
                }}
                $crate::Digest(out)
            }
            const fn write_blocks(&mut self, mut bytes: &[u8]) {
                if self.block_len != 0 {
                    let used = self.block_len as usize;
                    let free = $Self::BLOCK_LEN - used;
                    let take = $crate::cmp![min free, bytes.len()];
                    $crate::Slice::range_mut(&mut self.block, used, used + take)
                        .copy_from_slice($crate::Slice::range_to(&bytes, take));
                    self.block_len += take as u8;
                    bytes = $crate::Slice::range_from(&bytes, take);
                    if self.block_len as usize == $Self::BLOCK_LEN {
                        let block = self.block;
                        self.process_block(&block);
                        self.block_len = 0;
                    }
                }
                while bytes.len() >= $Self::BLOCK_LEN {
                    let mut block = [0u8; $Self::BLOCK_LEN];
                    block.copy_from_slice($crate::Slice::range_to(&bytes, $Self::BLOCK_LEN));
                    self.process_block(&block);
                    bytes = $crate::Slice::range_from(&bytes, $Self::BLOCK_LEN);
                }
                if !bytes.is_empty() {
                    $crate::Slice::range_to_mut(&mut self.block, bytes.len()).copy_from_slice(bytes);
                    self.block_len = bytes.len() as u8;
                }
            }
            const fn push_padding_byte(&mut self, byte: u8) {
                self.block[self.block_len as usize] = byte;
                self.block_len += 1;
                if self.block_len as usize == $Self::BLOCK_LEN {
                    let block = self.block;
                    self.process_block(&block);
                    self.block = [0; $Self::BLOCK_LEN];
                    self.block_len = 0;
                }
            }
            const fn process_block(&mut self, block: &[u8; $Self::BLOCK_LEN]) {
                let mut w = [0u32; 80];
                $crate::whilst! { t in 0..16; {
                    let i = t * 4;
                    w[t] = u32::from_be_bytes([block[i], block[i + 1], block[i + 2], block[i + 3]]);
                }}
                $crate::whilst! { t in 16..80; {
                    w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1);
                }}
                let [mut a, mut b, mut c, mut d, mut e] = self.state;
                $crate::whilst! { t in 0..80; {
                    let (f, k) = if t < 20 {
                        ((b & c) | ((!b) & d), 0x5A82_7999)
                    } else if t < 40 {
                        (b ^ c ^ d, 0x6ED9_EBA1)
                    } else if t < 60 {
                        ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC)
                    } else {
                        (b ^ c ^ d, 0xCA62_C1D6)
                    };
                    let temp = a.rotate_left(5)
                        .wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[t]);
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
        $crate::__crypto_impl_otp! { hash: $Self, otp: $crate::Otp, doc: "SHA-1" }
    };
}
#[doc(hidden)]
pub use __crypto_impl_sha1;
