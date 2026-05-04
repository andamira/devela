// devela::data::codec::crypto::md5
//
//! Defines [`Md5`].
//
// NOTES
// - Implements MD5 from the public algorithm specification.
// - This is not adapted from any third-party implementation.
// - References:

// use crate::{__crypto_impl_hmac, _impl_init, CryptoError, Digest, Slice, cmp, is, unwrap, whilst};

#[doc = crate::_tags!(crypto hash)]
/// Implements MD5 digest methods for a concrete hash type.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// MD5 is provided for compatibility with existing formats and protocols.
///
/// Do not use it for new security designs, digital signatures, password hashing,
/// collision-resistant identification, or integrity protection against attackers.
//
// References:
// - RFC 1321, The MD5 Message-Digest Algorithm.
// - RFC 6151, Updated Security Considerations for the MD5 Message-Digest
//   and the HMAC-MD5 Algorithms.
// - RFC 2104, Keyed-Hash Message Authentication Code.
// - RFC 2202, Test Cases for HMAC-MD5 and HMAC-SHA-1.
#[doc(hidden)]
#[macro_export]
macro_rules! __crypto_impl_md5 {
    ($(#[$attr:meta])* $vis:vis struct $Self:ident) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $vis struct $Self {
            state: [u32; 4],
            len_bits: u64,
            block: [u8; $Self::BLOCK_LEN],
            block_len: u8,
        }

        impl $crate::ConstInit for $Self { const INIT: $Self = $Self::new(); }
        impl Default for $Self { fn default() -> $Self { $Self::new() } }

        impl $Self {
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
            const S: [u32; 64] = [
                07, 12, 17, 22, 07, 12, 17, 22, 07, 12, 17, 22, 07, 12, 17, 22,
                05, 09, 14, 20, 05, 09, 14, 20, 05, 09, 14, 20, 05, 09, 14, 20,
                04, 11, 16, 23, 04, 11, 16, 23, 04, 11, 16, 23, 04, 11, 16, 23,
                06, 10, 15, 21, 06, 10, 15, 21, 06, 10, 15, 21, 06, 10, 15, 21,
            ];
            /// The digest size in bytes.
            pub const DIGEST_LEN: usize = 16;
            /// The internal block size in bytes.
            pub const BLOCK_LEN: usize = 64;

            /// Creates a new MD5 state.
            pub const fn new() -> $Self {
                $Self {
                    state: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476],
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
            /// if the total message length would exceed MD5's 64-bit bit-length field.
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
            /// Computes the MD5 digest of `bytes`.
            ///
            /// # Errors
            /// Returns [`LengthOverflow`][crate::CryptoError::LengthOverflow]
            /// if `bytes` exceeds MD5's supported message length.
            pub const fn digest_bytes(bytes: &[u8])
                -> Result<$crate::Digest<{ $Self::DIGEST_LEN }>, $crate::CryptoError> {
                let mut md5 = $Self::new();
                $crate::unwrap![ok? md5.update(bytes)];
                Ok(md5.finalize())
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
                let len = len_bits.to_le_bytes();
                $crate::whilst! { i in 0..len.len(); { self.push_padding_byte(len[i]); }}
                let mut out = [0u8; $Self::DIGEST_LEN];
                $crate::whilst! { i in 0..self.state.len(); {
                    let bytes = self.state[i].to_le_bytes();
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
                let mut m = [0u32; 16];
                $crate::whilst! { i in 0..16; {
                    let j = i * 4;
                    m[i] = u32::from_le_bytes([
                        block[j],
                        block[j + 1],
                        block[j + 2],
                        block[j + 3],
                    ]);
                }}
                let [mut a, mut b, mut c, mut d] = self.state;
                $crate::whilst! { i in 0..64; {
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
    };
}
#[doc(hidden)]
pub use __crypto_impl_md5;
