// devela::data::codec::crypto::sha
//
//! Defines SHA2 secure hash algorithms.
//
// TOC
// - impls:
//   - Sha256
//   - Sha512
//   - Sha224
//   - Sha384
//   - Sha512_224
//   - Sha512_384
// - macro: __crypto_impl_sha2!
// - consts: __SHA2_32_K, __SHA2_64_K

__crypto_impl_sha2! { word: u32, name: Sha256, doc: "SHA-256",
    digest: Sha256Digest, digest_bits: 256, digest_len: 32,
    output_words: 8, output_tail_bytes: 0,
    initial_state: [
        0x6A09_E667, 0xBB67_AE85, 0x3C6E_F372, 0xA54F_F53A,
        0x510E_527F, 0x9B05_688C, 0x1F83_D9AB, 0x5BE0_CD19,
    ]
}
__crypto_impl_sha2! { word: u64, name: Sha512, doc: "SHA-512",
    digest: Sha512Digest, digest_bits: 512, digest_len: 64,
    output_words: 8, output_tail_bytes: 0,
    initial_state: [
        0x6A09_E667_F3BC_C908, 0xBB67_AE85_84CA_A73B,
        0x3C6E_F372_FE94_F82B, 0xA54F_F53A_5F1D_36F1,
        0x510E_527F_ADE6_82D1, 0x9B05_688C_2B3E_6C1F,
        0x1F83_D9AB_FB41_BD6B, 0x5BE0_CD19_137E_2179,
    ]
}
crate::__crypto_impl_otp!(crate::Otp, Sha256, "SHA-256");
crate::__crypto_impl_otp!(crate::Otp, Sha512, "SHA-512");

__crypto_impl_sha2! { word: u32, name: Sha224, doc: "SHA-224",
    digest: Sha224Digest, digest_bits: 224, digest_len: 28,
    output_words: 7, output_tail_bytes: 0,
    initial_state: [
        0xC105_9ED8, 0x367C_D507, 0x3070_DD17, 0xF70E_5939,
        0xFFC0_0B31, 0x6858_1511, 0x64F9_8FA7, 0xBEFA_4FA4,
    ]
}
__crypto_impl_sha2! { word: u64, name: Sha384, doc: "SHA-384",
    digest: Sha384Digest, digest_bits: 384, digest_len: 48,
    output_words: 6, output_tail_bytes: 0,
    initial_state: [
        0xCBBB_9D5D_C105_9ED8, 0x629A_292A_367C_D507,
        0x9159_015A_3070_DD17, 0x152F_ECD8_F70E_5939,
        0x6733_2667_FFC0_0B31, 0x8EB4_4A87_6858_1511,
        0xDB0C_2E0D_64F9_8FA7, 0x47B5_481D_BEFA_4FA4,
    ]
}

__crypto_impl_sha2! { word: u64, name: Sha512_224, doc: "SHA-512/224",
    digest: Sha512_224Digest, digest_bits: 224, digest_len: 28,
    output_words: 3, output_tail_bytes: 4,
    initial_state: [
        0x8C3D_37C8_1954_4DA2, 0x73E1_9966_89DC_D4D6,
        0x1DFA_B7AE_32FF_9C82, 0x679D_D514_582F_9FCF,
        0x0F6D_2B69_7BD4_4DA8, 0x77E3_6F73_04C4_8942,
        0x3F9D_85A8_6A1D_36C8, 0x1112_E6AD_91D6_92A1,
    ]
}
__crypto_impl_sha2! { word: u64, name: Sha512_256, doc: "SHA-512/256",
    digest: Sha512_256Digest, digest_bits: 256, digest_len: 32,
    output_words: 4, output_tail_bytes: 0,
    initial_state: [
        0x2231_2194_FC2B_F72C, 0x9F55_5FA3_C84C_64C2,
        0x2393_B86B_6F53_B151, 0x9638_7719_5940_EABD,
        0x9628_3EE2_A88E_FFE3, 0xBE5E_1E25_5386_3992,
        0x2B01_99FC_2C85_B8AA, 0x0EB7_2DDC_81C5_2CA2,
    ]
}

/// Implements SHA-2 digest methods for a concrete hash type.
///
/// Supports the SHA-2 32-bit-word core used by SHA-224 and SHA-256, and the
/// SHA-2 64-bit-word core used by SHA-384, SHA-512, and SHA-512/t variants.
///
/// The selected word type determines the machine shape: word size, block size,
/// length-field size, schedule size, round constants, and compression functions.
/// The invocation supplies the concrete digest size, output truncation shape,
/// display name, and initial hash state.
//
// References:
// - FIPS 180-4, Secure Hash Standard.
// - RFC 6234, US Secure Hash Algorithms.
// - RFC 2104, Keyed-Hash Message Authentication Code.
// - RFC 4231, Test Vectors for HMAC-SHA-224, HMAC-SHA-256,
//   HMAC-SHA-384, and HMAC-SHA-512.
#[doc(hidden)]
#[macro_export]
macro_rules! __crypto_impl_sha2 {
    (word: u32,
     name: $Self:ident,
     doc: $DOC:literal,
     digest: $digest:ident,
     digest_bits: $digest_bits:literal,
     digest_len: $digest_len:literal,
     output_words: $output_words:literal,
     output_tail_bytes: $output_tail_bytes:tt,
     initial_state: [$($initial_state:literal),+ $(,)?]
    ) => {
        $crate::__crypto_impl_sha2! {
            %common
            name: $Self,
            doc: $DOC,
            digest: $digest,
            digest_bits: $digest_bits,
            digest_len: $digest_len,
            output_words: $output_words,
            output_tail_bytes: $output_tail_bytes,
            word: u32,
            len_bits: u64,
            block_len: 64,
            pad_at: 56,
            schedule_len: 64,
            word_bytes: 4,
            full_len: 32,
            k: $crate::__SHA2_32_K,
            initial_state: [$($initial_state),+]
        }
        impl $Self {
            const fn ch(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ ((!x) & z) }
            const fn maj(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (x & z) ^ (y & z) }
            const fn big_sigma0(x: u32) -> u32 {
                x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22) }
            const fn big_sigma1(x: u32) -> u32 {
                x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25) }
            const fn small_sigma0(x: u32) -> u32 {
                x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3) }
            const fn small_sigma1(x: u32) -> u32 {
                x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10) }
        }
    };
    (word: u64,
     name: $Self:ident,
     doc: $DOC:literal,
     digest: $digest:ident,
     digest_bits: $digest_bits:literal,
     digest_len: $digest_len:literal,
     output_words: $output_words:literal,
     output_tail_bytes: $output_tail_bytes:tt,
     initial_state: [$($initial_state:literal),+ $(,)?]
    ) => {
        $crate::__crypto_impl_sha2! {
            %common
            name: $Self,
            doc: $DOC,
            digest: $digest,
            digest_bits: $digest_bits,
            digest_len: $digest_len,
            output_words: $output_words,
            output_tail_bytes: $output_tail_bytes,
            word: u64,
            len_bits: u128,
            block_len: 128,
            pad_at: 112,
            schedule_len: 80,
            word_bytes: 8,
            full_len: 64,
            k: $crate::__SHA2_64_K,
            initial_state: [$($initial_state),+]
        }
        impl $Self {
            const fn ch(x: u64, y: u64, z: u64) -> u64 { (x & y) ^ ((!x) & z) }
            const fn maj(x: u64, y: u64, z: u64) -> u64 { (x & y) ^ (x & z) ^ (y & z) }
            const fn big_sigma0(x: u64) -> u64 {
                x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39) }
            const fn big_sigma1(x: u64) -> u64 {
                x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41) }
            const fn small_sigma0(x: u64) -> u64 {
                x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7) }
            const fn small_sigma1(x: u64) -> u64 {
                x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6) }
        }
    };
    (%common
     name: $Self:ident,
     doc: $DOC:literal,
     digest: $digest:ident,
     digest_bits: $digest_bits:literal,
     digest_len: $digest_len:literal,
     output_words: $output_words:literal,
     output_tail_bytes: $output_tail_bytes:tt,
     word: $word:tt,
     len_bits: $len_bits:ty,
     block_len: $block_len:literal,
     pad_at: $pad_at:literal,
     schedule_len: $schedule_len:literal,
     word_bytes: $word_bytes:tt,
     full_len: $full_len:literal,
     k: $K:path,
     initial_state: [$($initial_state:literal),+ $(,)?]
    ) => {
        #[doc = crate::_tags!(crypto hash)]
        #[doc = concat!("Incremental ", $DOC, " state.")]
        #[doc = crate::_doc_location!("data/codec/crypto")]
        ///
        #[doc = concat!($DOC, " is provided as a fixed-size cryptographic message digest.")]
        ///
        /// This implementation is allocation-free, compile-time friendly,
        /// and supports incremental updates.
        ///
        /// Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize] to consume
        #[doc = concat!("the state and return the ", stringify!($digest_len) , "-byte digest.")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $Self {
            state: [$word; 8],
            len_bits: $len_bits,
            block: [u8; $Self::BLOCK_LEN],
            block_len: u8,
        }
        pub(crate) type $digest = $crate::Digest<{ $Self::DIGEST_LEN }>;

        $crate::_impl_init![ConstInit: $Self::new() => $Self];
        impl Default for $Self { fn default() -> $Self { $Self::new() } }

        impl $Self {
            const __: [(); $digest_len] = [(); $output_words * $word_bytes + $output_tail_bytes];

            /// The digest size in bytes.
            pub const DIGEST_LEN: usize = $digest_len;
            /// The internal block size in bytes.
            pub const BLOCK_LEN: usize = $block_len;

            #[doc = concat!("Creates a new ", $DOC, " state.")]
            pub const fn new() -> Self {
                Self {
                    state: [ $($initial_state),+ ],
                    len_bits: 0,
                    block: [0; $Self::BLOCK_LEN],
                    block_len: 0,
                }
            }
            /// Returns the number of message bits already accepted.
            pub const fn len_bits(&self) -> $len_bits { self.len_bits }
            /// Returns `true` if no message bytes have been accepted.
            pub const fn is_empty(&self) -> bool { self.len_bits == 0 }
            /// Resets the state to its initial value.
            pub const fn reset(&mut self) { *self = $Self::new(); }

            /// Updates the digest state with `bytes`.
            ///
            /// # Errors
            /// Returns [`LengthOverflow`] if the total message length would exceed
            #[doc = concat!($DOC, "'s ", stringify!($len_bits), " bit-length field.")]
            ///
            /// [`LengthOverflow`]: crate::CryptoError::LengthOverflow
            pub const fn update(&mut self, bytes: &[u8]) -> Result<(), $crate::CryptoError> {
                let Some(add_bits) = (bytes.len() as $len_bits).checked_mul(8) else {
                    return Err($crate::CryptoError::LengthOverflow);
                };
                let Some(new_len_bits) = self.len_bits.checked_add(add_bits) else {
                    return Err($crate::CryptoError::LengthOverflow);
                };
                self.len_bits = new_len_bits;
                self.write_blocks(bytes);
                Ok(())
            }
            #[doc = concat!("Computes the ", $DOC, " digest of `bytes`.")]
            ///
            /// # Errors
            /// Returns [`LengthOverflow`]
            #[doc = concat!("if `bytes` exceeds ", $DOC, "'s supported message length.")]
            ///
            /// [`LengthOverflow`]: crate::CryptoError::LengthOverflow
            pub const fn digest_bytes(bytes: &[u8]) -> Result<$digest, $crate::CryptoError> {
                let mut sha = Self::new();
                $crate::unwrap![ok? sha.update(bytes)];
                Ok(sha.finalize())
            }

            $crate::__crypto_impl_hmac![$Self, $digest];

            /// Finalizes the digest and consumes the state.
            ///
            /// This method cannot fail.
            pub const fn finalize(mut self) -> $digest {
                let len_bits = self.len_bits;
                self.push_padding_byte(0x80);
                while self.block_len != $pad_at { self.push_padding_byte(0); }
                let len = len_bits.to_be_bytes();
                $crate::whilst! { i in 0..len.len(); { self.push_padding_byte(len[i]); }}
                let mut out = [0u8; $digest_len];
                $crate::__crypto_impl_sha2! { %write_digest
                    out: out, state: self.state,
                    word_bytes: $word_bytes,
                    output_words: $output_words,
                    output_tail_bytes: $output_tail_bytes
                }
                $crate::Digest(out)
            }

            /* private helpers*/
            const K: [$word; $schedule_len] = $K;

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
                    $crate::Slice::range_to_mut(&mut self.block, bytes.len())
                        .copy_from_slice(bytes);
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
                let mut w = [0; $schedule_len];
                $crate::whilst! { t in 0..16; {
                    let i = t * $word_bytes;
                    w[t] = $crate::__crypto_impl_sha2![%read_word word: $word, block: block, at: i];
                }}
                $crate::whilst! { t in 16..$schedule_len; {
                    w[t] = Self::small_sigma1(w[t - 2])
                        .wrapping_add(w[t - 7])
                        .wrapping_add(Self::small_sigma0(w[t - 15]))
                        .wrapping_add(w[t - 16]);
                }}
                let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
                $crate::whilst! { t in 0..$schedule_len; {
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
        }
    };
    (%write_digest
        out: $out:ident,
        state: $state:expr,
        word_bytes: $word_bytes:tt,
        output_words: $output_words:literal,
        output_tail_bytes: $output_tail_bytes:tt
    ) => {
        $crate::whilst! { wi in 0..$output_words; {
            let bytes = ($state)[wi].to_be_bytes();
            let j = wi * $word_bytes;
            $crate::punroll! { $word_bytes |bi|
                $out[j + bi] = bytes[bi]
            }
        }}
        $crate::__crypto_impl_sha2! { %write_digest_tail
            out: $out,
            state: $state,
            word_bytes: $word_bytes,
            output_words: $output_words,
            output_tail_bytes: $output_tail_bytes
        }
    };
    (%write_digest_tail
        out: $out:ident,
        state: $state:expr,
        word_bytes: $word_bytes:tt,
        output_words: $output_words:literal,
        output_tail_bytes: 0
    ) => {};
    (%write_digest_tail
        out: $out:ident,
        state: $state:expr,
        word_bytes: $word_bytes:tt,
        output_words: $output_words:literal,
        output_tail_bytes: $tail:tt
    ) => {
        let bytes = ($state)[$output_words].to_be_bytes();
        let j = $output_words * $word_bytes;
        $crate::punroll! { $tail |bi| $out[j + bi] = bytes[bi] }
    };
    (%read_word word: u32, block: $block:expr, at: $i:expr) => {
        u32::from_be_bytes([
            $block[$i],
            $block[$i + 1],
            $block[$i + 2],
            $block[$i + 3],
        ])
    };
    (%read_word word: u64, block: $block:expr, at: $i:expr) => {
        u64::from_be_bytes([
            $block[$i],
            $block[$i + 1],
            $block[$i + 2],
            $block[$i + 3],
            $block[$i + 4],
            $block[$i + 5],
            $block[$i + 6],
            $block[$i + 7],
        ])
    };
}
#[doc(hidden)]
pub use __crypto_impl_sha2;

/// SHA-2 32-bit-word round constants.
///
/// These are the first 32 bits of the fractional parts of the cube roots
/// of the first 64 prime numbers. One fixed constant is mixed into each
/// compression round and shared by SHA-224 and SHA-256.
#[doc(hidden)]
#[rustfmt::skip]
pub const __SHA2_32_K: [u32; 64] = [
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

/// SHA-2 64-bit-word round constants.
///
/// These are the first 64 bits of the fractional parts of the cube roots
/// of the first 80 prime numbers. One fixed constant is mixed into each
/// compression round and shared by SHA-384, SHA-512, and SHA-512/t variants.
#[doc(hidden)]
#[rustfmt::skip]
pub const __SHA2_64_K: [u64; 80] = [
    0x428A_2F98_D728_AE22, 0x7137_4491_23EF_65CD, 0xB5C0_FBCF_EC4D_3B2F, 0xE9B5_DBA5_8189_DBBC,
    0x3956_C25B_F348_B538, 0x59F1_11F1_B605_D019, 0x923F_82A4_AF19_4F9B, 0xAB1C_5ED5_DA6D_8118,
    0xD807_AA98_A303_0242, 0x1283_5B01_4570_6FBE, 0x2431_85BE_4EE4_B28C, 0x550C_7DC3_D5FF_B4E2,
    0x72BE_5D74_F27B_896F, 0x80DE_B1FE_3B16_96B1, 0x9BDC_06A7_25C7_1235, 0xC19B_F174_CF69_2694,
    0xE49B_69C1_9EF1_4AD2, 0xEFBE_4786_384F_25E3, 0x0FC1_9DC6_8B8C_D5B5, 0x240C_A1CC_77AC_9C65,
    0x2DE9_2C6F_592B_0275, 0x4A74_84AA_6EA6_E483, 0x5CB0_A9DC_BD41_FBD4, 0x76F9_88DA_8311_53B5,
    0x983E_5152_EE66_DFAB, 0xA831_C66D_2DB4_3210, 0xB003_27C8_98FB_213F, 0xBF59_7FC7_BEEF_0EE4,
    0xC6E0_0BF3_3DA8_8FC2, 0xD5A7_9147_930A_A725, 0x06CA_6351_E003_826F, 0x1429_2967_0A0E_6E70,
    0x27B7_0A85_46D2_2FFC, 0x2E1B_2138_5C26_C926, 0x4D2C_6DFC_5AC4_2AED, 0x5338_0D13_9D95_B3DF,
    0x650A_7354_8BAF_63DE, 0x766A_0ABB_3C77_B2A8, 0x81C2_C92E_47ED_AEE6, 0x9272_2C85_1482_353B,
    0xA2BF_E8A1_4CF1_0364, 0xA81A_664B_BC42_3001, 0xC24B_8B70_D0F8_9791, 0xC76C_51A3_0654_BE30,
    0xD192_E819_D6EF_5218, 0xD699_0624_5565_A910, 0xF40E_3585_5771_202A, 0x106A_A070_32BB_D1B8,
    0x19A4_C116_B8D2_D0C8, 0x1E37_6C08_5141_AB53, 0x2748_774C_DF8E_EB99, 0x34B0_BCB5_E19B_48A8,
    0x391C_0CB3_C5C9_5A63, 0x4ED8_AA4A_E341_8ACB, 0x5B9C_CA4F_7763_E373, 0x682E_6FF3_D6B2_B8A3,
    0x748F_82EE_5DEF_B2FC, 0x78A5_636F_4317_2F60, 0x84C8_7814_A1F0_AB72, 0x8CC7_0208_1A64_39EC,
    0x90BE_FFFA_2363_1E28, 0xA450_6CEB_DE82_BDE9, 0xBEF9_A3F7_B2C6_7915, 0xC671_78F2_E372_532B,
    0xCA27_3ECE_EA26_619C, 0xD186_B8C7_21C0_C207, 0xEADA_7DD6_CDE0_EB1E, 0xF57D_4F7F_EE6E_D178,
    0x06F0_67AA_7217_6FBA, 0x0A63_7DC5_A2C8_98A6, 0x113F_9804_BEF9_0DAE, 0x1B71_0B35_131C_471B,
    0x28DB_77F5_2304_7D84, 0x32CA_AB7B_40C7_2493, 0x3C9E_BE0A_15C9_BEBC, 0x431D_67C4_9C10_0D4C,
    0x4CC5_D4BE_CB3E_42B6, 0x597F_299C_FC65_7E2A, 0x5FCB_6FAB_3AD6_FAEC, 0x6C44_198C_4A47_5817,
];
