// devela::data::codec::crypto::sha512
//
//! Defines [`Sha512`].
//
// NOTES
// - Implements SHA-512 from the public algorithm specification.
// - This is not adapted from any third-party implementation.
// - References:
//   - FIPS 180-4, Secure Hash Standard.
//   - RFC 6234, US Secure Hash Algorithms.
//   - RFC 2104, Keyed-Hash Message Authentication Code.
//   - RFC 4231, Identifiers and Test Vectors for HMAC-SHA-224,
//     HMAC-SHA-256, HMAC-SHA-384, and HMAC-SHA-512.

use crate::{_crypto_impl_hmac, _impl_init, CryptoError, Digest, Slice, cmp, unwrap, whilst};

type Sha512Digest = Digest<{ Sha512::DIGEST_LEN }>;

#[doc = crate::_tags!(crypto hash)]
/// Incremental SHA-512 state.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// SHA-512 is provided as a fixed-size cryptographic message digest.
///
/// This implementation is allocation-free, compile-time friendly,
/// and supports incremental updates.
///
/// Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize]
/// to consume the state and return the 64-byte digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sha512 {
    state: [u64; 8],
    len_bits: u128,
    block: [u8; Sha512::BLOCK_LEN],
    block_len: u8,
}
_impl_init![ConstInit: Self::new() => Sha512];
impl Default for Sha512 {
    fn default() -> Self {
        Self::new()
    }
}
impl Sha512 {
    /// The digest size in bytes.
    pub const DIGEST_LEN: usize = 64;
    /// The internal block size in bytes.
    pub const BLOCK_LEN: usize = 128;
    #[rustfmt::skip]
    const K: [u64; 80] = [
        0x428A_2F98_D728_AE22, 0x7137_4491_23EF_65CD,
        0xB5C0_FBCF_EC4D_3B2F, 0xE9B5_DBA5_8189_DBBC,
        0x3956_C25B_F348_B538, 0x59F1_11F1_B605_D019,
        0x923F_82A4_AF19_4F9B, 0xAB1C_5ED5_DA6D_8118,
        0xD807_AA98_A303_0242, 0x1283_5B01_4570_6FBE,
        0x2431_85BE_4EE4_B28C, 0x550C_7DC3_D5FF_B4E2,
        0x72BE_5D74_F27B_896F, 0x80DE_B1FE_3B16_96B1,
        0x9BDC_06A7_25C7_1235, 0xC19B_F174_CF69_2694,
        0xE49B_69C1_9EF1_4AD2, 0xEFBE_4786_384F_25E3,
        0x0FC1_9DC6_8B8C_D5B5, 0x240C_A1CC_77AC_9C65,
        0x2DE9_2C6F_592B_0275, 0x4A74_84AA_6EA6_E483,
        0x5CB0_A9DC_BD41_FBD4, 0x76F9_88DA_8311_53B5,
        0x983E_5152_EE66_DFAB, 0xA831_C66D_2DB4_3210,
        0xB003_27C8_98FB_213F, 0xBF59_7FC7_BEEF_0EE4,
        0xC6E0_0BF3_3DA8_8FC2, 0xD5A7_9147_930A_A725,
        0x06CA_6351_E003_826F, 0x1429_2967_0A0E_6E70,
        0x27B7_0A85_46D2_2FFC, 0x2E1B_2138_5C26_C926,
        0x4D2C_6DFC_5AC4_2AED, 0x5338_0D13_9D95_B3DF,
        0x650A_7354_8BAF_63DE, 0x766A_0ABB_3C77_B2A8,
        0x81C2_C92E_47ED_AEE6, 0x9272_2C85_1482_353B,
        0xA2BF_E8A1_4CF1_0364, 0xA81A_664B_BC42_3001,
        0xC24B_8B70_D0F8_9791, 0xC76C_51A3_0654_BE30,
        0xD192_E819_D6EF_5218, 0xD699_0624_5565_A910,
        0xF40E_3585_5771_202A, 0x106A_A070_32BB_D1B8,
        0x19A4_C116_B8D2_D0C8, 0x1E37_6C08_5141_AB53,
        0x2748_774C_DF8E_EB99, 0x34B0_BCB5_E19B_48A8,
        0x391C_0CB3_C5C9_5A63, 0x4ED8_AA4A_E341_8ACB,
        0x5B9C_CA4F_7763_E373, 0x682E_6FF3_D6B2_B8A3,
        0x748F_82EE_5DEF_B2FC, 0x78A5_636F_4317_2F60,
        0x84C8_7814_A1F0_AB72, 0x8CC7_0208_1A64_39EC,
        0x90BE_FFFA_2363_1E28, 0xA450_6CEB_DE82_BDE9,
        0xBEF9_A3F7_B2C6_7915, 0xC671_78F2_E372_532B,
        0xCA27_3ECE_EA26_619C, 0xD186_B8C7_21C0_C207,
        0xEADA_7DD6_CDE0_EB1E, 0xF57D_4F7F_EE6E_D178,
        0x06F0_67AA_7217_6FBA, 0x0A63_7DC5_A2C8_98A6,
        0x113F_9804_BEF9_0DAE, 0x1B71_0B35_131C_471B,
        0x28DB_77F5_2304_7D84, 0x32CA_AB7B_40C7_2493,
        0x3C9E_BE0A_15C9_BEBC, 0x431D_67C4_9C10_0D4C,
        0x4CC5_D4BE_CB3E_42B6, 0x597F_299C_FC65_7E2A,
        0x5FCB_6FAB_3AD6_FAEC, 0x6C44_198C_4A47_5817,
    ];
    /// Creates a new SHA-512 state.
    #[rustfmt::skip]
    pub const fn new() -> Self {
        Self {
            state: [
                0x6A09_E667_F3BC_C908, 0xBB67_AE85_84CA_A73B,
                0x3C6E_F372_FE94_F82B, 0xA54F_F53A_5F1D_36F1,
                0x510E_527F_ADE6_82D1, 0x9B05_688C_2B3E_6C1F,
                0x1F83_D9AB_FB41_BD6B, 0x5BE0_CD19_137E_2179,
            ],
            len_bits: 0,
            block: [0; Sha512::BLOCK_LEN],
            block_len: 0,
        }
    }
    /// Returns the number of message bits already accepted.
    pub const fn len_bits(&self) -> u128 {
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
    /// would exceed SHA-512's 128-bit bit-length field.
    pub const fn update(&mut self, bytes: &[u8]) -> Result<(), CryptoError> {
        let add_bits = (bytes.len() as u128) * 8;
        let Some(new_len_bits) = self.len_bits.checked_add(add_bits) else {
            return Err(CryptoError::LengthOverflow);
        };
        self.len_bits = new_len_bits;
        self.write_blocks(bytes);
        Ok(())
    }
    /// Computes the SHA-512 digest of `bytes`.
    ///
    /// # Errors
    /// Returns [`CryptoError::LengthOverflow`] if `bytes` exceeds SHA-512's
    /// supported message length.
    pub const fn digest_bytes(bytes: &[u8]) -> Result<Sha512Digest, CryptoError> {
        let mut sha = Self::new();
        unwrap![ok? sha.update(bytes)];
        Ok(sha.finalize())
    }

    _crypto_impl_hmac![Sha512, Sha512Digest, "HMAC-SHA-512"];

    /// Finalizes the digest and consumes the state.
    ///
    /// This method cannot fail.
    pub const fn finalize(mut self) -> Sha512Digest {
        let len_bits = self.len_bits;
        self.push_padding_byte(0x80);
        while self.block_len != 112 {
            self.push_padding_byte(0);
        }
        let len = len_bits.to_be_bytes();
        whilst! { i in 0..len.len(); { self.push_padding_byte(len[i]); }}
        let mut out = [0u8; Sha512::DIGEST_LEN];
        whilst! { i in 0..self.state.len(); {
            let bytes = self.state[i].to_be_bytes();
            let j = i * 8;
            out[j] = bytes[0];
            out[j + 1] = bytes[1];
            out[j + 2] = bytes[2];
            out[j + 3] = bytes[3];
            out[j + 4] = bytes[4];
            out[j + 5] = bytes[5];
            out[j + 6] = bytes[6];
            out[j + 7] = bytes[7];
        }}
        Digest(out)
    }
    const fn write_blocks(&mut self, mut bytes: &[u8]) {
        if self.block_len != 0 {
            let used = self.block_len as usize;
            let free = Sha512::BLOCK_LEN - used;
            let take = cmp![min free, bytes.len()];
            Slice::range_mut(&mut self.block, used, used + take)
                .copy_from_slice(Slice::range_to(&bytes, take));
            self.block_len += take as u8;
            bytes = Slice::range_from(&bytes, take);
            if self.block_len as usize == Sha512::BLOCK_LEN {
                let block = self.block;
                self.process_block(&block);
                self.block_len = 0;
            }
        }
        while bytes.len() >= Sha512::BLOCK_LEN {
            let mut block = [0u8; Sha512::BLOCK_LEN];
            block.copy_from_slice(Slice::range_to(&bytes, Sha512::BLOCK_LEN));
            self.process_block(&block);
            bytes = Slice::range_from(&bytes, Sha512::BLOCK_LEN);
        }
        if !bytes.is_empty() {
            Slice::range_to_mut(&mut self.block, bytes.len()).copy_from_slice(bytes);
            self.block_len = bytes.len() as u8;
        }
    }
    const fn push_padding_byte(&mut self, byte: u8) {
        self.block[self.block_len as usize] = byte;
        self.block_len += 1;
        if self.block_len as usize == Sha512::BLOCK_LEN {
            let block = self.block;
            self.process_block(&block);
            self.block = [0; Sha512::BLOCK_LEN];
            self.block_len = 0;
        }
    }
    const fn process_block(&mut self, block: &[u8; Sha512::BLOCK_LEN]) {
        let mut w = [0u64; 80];
        whilst! { t in 0..16; {
            let i = t * 8;
            w[t] = u64::from_be_bytes([
                block[i],
                block[i + 1],
                block[i + 2],
                block[i + 3],
                block[i + 4],
                block[i + 5],
                block[i + 6],
                block[i + 7],
            ]);
        }}
        whilst! { t in 16..80; {
            w[t] = Self::small_sigma1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(Self::small_sigma0(w[t - 15]))
                .wrapping_add(w[t - 16]);
        }}
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        whilst! { t in 0..80; {
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
    const fn ch(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ ((!x) & z)
    }
    const fn maj(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (x & z) ^ (y & z)
    }
    const fn big_sigma0(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }
    const fn big_sigma1(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }
    const fn small_sigma0(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
    }
    const fn small_sigma1(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::_hex, Digest, Sha512, Sha512Digest, whilst};

    fn digest_from_hex(hex: &str) -> Sha512Digest {
        Digest(self::_hex(hex))
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Sha512::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        assert_digest(
            b"",
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
        );
        assert_digest(
            b"abc",
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        );
        assert_digest(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "204a8fc6dda82f0a0ced7beb8e08a41657c16ef468b228a8279be331a703c335\
            96fd15c13b1b07f9aa1d3bea57789ca031ad85c7a71dd70354ec631238ca3445",
        );
    }
    #[test]
    #[cfg(not(miri))] // too slow for miri
    fn million_a() {
        let mut sha = Sha512::new();
        for _ in 0..1_000_000 {
            sha.update(b"a").unwrap();
        }
        assert_eq!(
            sha.finalize(),
            digest_from_hex(
                "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973eb\
                 de0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b",
            ),
        );
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        // 1
        let key = [0x0b; 20];
        assert_eq!(
            Sha512::hmac(&key, b"Hi There").unwrap(),
            digest_from_hex(
                "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
                 daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854",
            ),
        );
        // 2
        assert_eq!(
            Sha512::hmac(b"Jefe", b"what do ya want for nothing?").unwrap(),
            digest_from_hex(
                "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea250554\
                 9758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737",
            ),
        );
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Sha512::digest_bytes(input).unwrap();
        let mut chunked = Sha512::new();
        chunked.update(b"the quick ").unwrap();
        chunked.update(b"brown fox ").unwrap();
        chunked.update(b"jumps over ").unwrap();
        chunked.update(b"the lazy dog").unwrap();
        assert_eq!(full, chunked.finalize());
    }
    #[test]
    fn boundary_lengths() {
        for len in [0, 1, 111, 112, 113, 127, 128, 129, 239, 240] {
            let mut bytes = [0u8; 240];
            whilst! { i in 0..len; { bytes[i] = i as u8; }}
            let full = Sha512::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Sha512::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut sha = Sha512::new();
        sha.update(b"abc").unwrap();
        sha.reset();
        sha.update(b"abc").unwrap();
        assert_eq!(
            sha.finalize(),
            digest_from_hex(
                "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
                 2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
            ),
        );
    }
}
