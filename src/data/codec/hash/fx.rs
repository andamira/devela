// devela/src/data/codec/hash/fx.rs
//
//! Defines [`HasherBuildFx`], [`HasherFx`].
//

use crate::{ConstInit, Hash, Hasher, HasherBuildDefault, is, lets};

#[doc = crate::_tags!(hash init)]
/// A builder for default Fx hashers.
#[doc = crate::_doc_meta!{
    location("data/codec/hash", type HasherBuildFx),
}]
pub type HasherBuildFx = HasherBuildDefault<HasherFx<usize>>;

#[doc = crate::_tags!(hash)]
/// A fast non-cryptographic Fx hasher based on the algorithm used by rustc.
#[doc = crate::_doc_meta!{
    location("data/codec/hash", struct HasherFx),
    test_size_of(HasherFx<u64> = 8|64; niche !Option),
}]
/// `usize` is target-native; `u32` and `u64` provide fixed-width variants.
///
/// Integer values use polynomial mixing,
/// while byte slices use a wyhash-inspired compressor.
///
/// Hash outputs are not stable
/// and are unsuitable for cryptographic or adversarial use.
#[doc = crate::_doc_vendor!("rustc-hash")]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HasherFx<T> {
    state: T,
}

/* constants */

/// Multiplicative constant used by the 32-bit Fx state.
const K32: u32 = 0x93d7_65dd;
/// Multiplicative constant used by the 64-bit Fx state.
const K64: u64 = 0xf135_7aea_2e62_a9c5;

#[cfg(target_pointer_width = "32")]
const K: usize = K32 as usize;
#[cfg(target_pointer_width = "64")]
const K: usize = K64 as usize;

/// Final rotation for the 32-bit state.
const ROTATE32: u32 = 15;
/// Final rotation for the 64-bit state.
const ROTATE64: u32 = 26;

#[cfg(target_pointer_width = "32")]
const ROTATE: u32 = ROTATE32;
#[cfg(target_pointer_width = "64")]
const ROTATE: u32 = ROTATE64;

// Digits of pi.
const SEED1: u64 = 0x243f_6a88_85a3_08d3;
const SEED2: u64 = 0x1319_8a2e_0370_7344;

/// Prevents an immediate trivial collapse when hashing common zero-filled data.
const PREVENT_TRIVIAL_ZERO_COLLAPSE: u64 = 0xa409_3822_299f_31d0;

/// Whether the target efficiently supports widening `u64 × u64 → u128` multiplication.
const WIDE_MIX: bool = cfg!(any(
    all(target_pointer_width = "64", not(any(target_arch = "sparc64", target_arch = "wasm64"))),
    target_arch = "aarch64",
    target_arch = "x86_64",
    all(target_family = "wasm", target_feature = "wide-arithmetic"),
));

/* common state implementation */

macro_rules! impl_fx {
    () => { impl_fx![u32: K32, ROTATE32; u64: K64, ROTATE64; usize: K, ROTATE]; };
    ($($t:ty: $k:ident, $rotate:ident);+ $(;)?) => {
        $(
            impl ConstInit for HasherFx<$t> { const INIT: Self = Self { state: 0 }; }
            impl Default for HasherFx<$t> { fn default() -> Self { Self::INIT } }

            impl HasherFx<$t> {
                /// Creates a zero-seeded hasher.
                pub const fn new() -> Self { Self::INIT }

                /// Creates a hasher initialized with `seed`.
                pub const fn with_seed(seed: $t) -> Self { Self { state: seed } }

                /// Hashes `value` with the default seed.
                pub fn hash<T: Hash + ?Sized>(value: &T) -> $t {
                    let mut hasher = Self::new();
                    value.hash(&mut hasher);
                    Self::finalize(hasher.state)
                }
                /* private */

                /// Advances an Fx state with one native state word.
                const fn add_to_hash(hash: $t, word: $t) -> $t {
                    hash.wrapping_add(word).wrapping_mul($k)
                }
                /// Move high-entropy bits into the low positions used by hash tables.
                const fn finalize(hash: $t) -> $t {
                    hash.rotate_left($rotate)
                }
            }
        )+
    };
}
impl_fx!();

/* helpers */

/// Reads one little-endian `u32` beginning at `offset`.
///
/// The caller guarantees that four bytes remain.
const fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    (bytes[offset] as u32)
        | ((bytes[offset + 1] as u32) << 8)
        | ((bytes[offset + 2] as u32) << 16)
        | ((bytes[offset + 3] as u32) << 24)
}
/// Reads one little-endian `u64` beginning at `offset`.
///
/// The caller guarantees that eight bytes remain.
const fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    (bytes[offset] as u64)
        | ((bytes[offset + 1] as u64) << 8)
        | ((bytes[offset + 2] as u64) << 16)
        | ((bytes[offset + 3] as u64) << 24)
        | ((bytes[offset + 4] as u64) << 32)
        | ((bytes[offset + 5] as u64) << 40)
        | ((bytes[offset + 6] as u64) << 48)
        | ((bytes[offset + 7] as u64) << 56)
}

/// Mixes two `u64` words, using widening multiplication when `WIDE` is true.
const fn multiply_mix<const WIDE: bool>(x: u64, y: u64) -> u64 {
    if WIDE {
        let full = (x as u128).wrapping_mul(y as u128);
        let lo = full as u64;
        let hi = (full >> 64) as u64;
        lo ^ hi
    } else {
        lets! { lx = x as u32, ly = y as u32 }
        lets! { hx = (x >> 32) as u32, hy = (y >> 32) as u32 }
        let a = (lx as u64).wrapping_mul(hy as u64);
        let b = (hx as u64).wrapping_mul(ly as u64);
        a ^ b.rotate_right(32)
    }
}

/// Compresses an arbitrary byte slice to one 64-bit word.
///
/// This is the wyhash-inspired byte path used by modern `rustc-hash`.
const fn hash_bytes<const WIDE: bool>(bytes: &[u8]) -> u64 {
    let len = bytes.len();
    lets! { mut s0 = SEED1, mut s1 = SEED2 }
    if len <= 16 {
        if len >= 8 {
            s0 ^= read_u64_le(bytes, 0);
            s1 ^= read_u64_le(bytes, len - 8);
        } else if len >= 4 {
            s0 ^= read_u32_le(bytes, 0) as u64;
            s1 ^= read_u32_le(bytes, len - 4) as u64;
        } else if len > 0 {
            let lo = bytes[0];
            let mid = bytes[len / 2];
            let hi = bytes[len - 1];
            s0 ^= lo as u64;
            s1 ^= ((hi as u64) << 8) | mid as u64;
        }
    } else {
        let mut cursor = 0;
        while cursor + 16 < len {
            let x = read_u64_le(bytes, cursor);
            let y = read_u64_le(bytes, cursor + 8);
            let mixed = multiply_mix::<WIDE>(s0 ^ x, PREVENT_TRIVIAL_ZERO_COLLAPSE ^ y);
            s0 = s1;
            s1 = mixed;
            cursor += 16;
        }
        let suffix = len - 16;
        s0 ^= read_u64_le(bytes, suffix);
        s1 ^= read_u64_le(bytes, suffix + 8);
    }
    multiply_mix::<WIDE>(s0, s1) ^ len as u64
}

/* const impls */

impl HasherFx<u32> {
    /// Hashes a byte slice with the default seed.
    pub const fn hash_bytes(bytes: &[u8]) -> u32 {
        Self::hash_bytes_with_seed(0, bytes)
    }
    /// Hashes a byte slice with `seed`.
    pub const fn hash_bytes_with_seed(seed: u32, bytes: &[u8]) -> u32 {
        let compressed = hash_bytes::<false>(bytes);
        let mut state = seed;
        state = Self::add_to_hash(state, compressed as u32);
        state = Self::add_to_hash(state, (compressed >> 32) as u32);
        Self::finalize(state)
    }
}
impl HasherFx<u64> {
    /// Hashes a byte slice with the default seed.
    pub const fn hash_bytes(bytes: &[u8]) -> u64 {
        Self::hash_bytes_with_seed(0, bytes)
    }
    /// Hashes a byte slice with `seed`.
    pub const fn hash_bytes_with_seed(seed: u64, bytes: &[u8]) -> u64 {
        let compressed = hash_bytes::<true>(bytes);
        let state = Self::add_to_hash(seed, compressed);
        Self::finalize(state)
    }
}
impl HasherFx<usize> {
    /// Hashes a byte slice with the default seed.
    pub const fn hash_bytes(bytes: &[u8]) -> usize {
        Self::hash_bytes_with_seed(0, bytes)
    }
    /// Hashes a byte slice with `seed`.
    pub const fn hash_bytes_with_seed(seed: usize, bytes: &[u8]) -> usize {
        let compressed = hash_bytes::<WIDE_MIX>(bytes);
        #[allow(unused_mut)]
        let mut state = Self::add_to_hash(seed, compressed as usize);
        #[cfg(target_pointer_width = "32")]
        {
            state = Self::add_to_hash(state, (compressed >> 32) as usize);
        }
        Self::finalize(state)
    }
    /// Hashes a primitive's little-endian bytes through the integer fast path.
    ///
    /// Use [`Self::hash_bytes`] for arbitrary byte slices.
    pub const fn hash_primitive_bytes(bytes: &[u8]) -> usize {
        lets! { mut state = 0, mut cursor = 0 }
        #[cfg(target_pointer_width = "64")]
        while bytes.len() - cursor >= 8 {
            state = Self::add_to_hash(state, read_u64_le(bytes, cursor) as usize);
            cursor += 8;
        }
        while bytes.len() - cursor >= 4 {
            state = Self::add_to_hash(state, read_u32_le(bytes, cursor) as usize);
            cursor += 4;
        }
        if bytes.len() - cursor >= 2 {
            state = Self::add_to_hash(
                state,
                u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]) as usize,
            );
            cursor += 2;
        }
        is! { bytes.len() - cursor != 0, state = Self::add_to_hash(state, bytes[cursor] as usize) }
        Self::finalize(state)
    }
}

/* impl traits */

impl Hasher for HasherFx<u32> {
    fn write(&mut self, bytes: &[u8]) {
        let compressed = hash_bytes::<false>(bytes);
        self.state = Self::add_to_hash(self.state, compressed as u32);
        self.state = Self::add_to_hash(self.state, (compressed >> 32) as u32);
    }
    fn write_u8(&mut self, i: u8) {
        self.state = Self::add_to_hash(self.state, i as u32);
    }
    fn write_u16(&mut self, i: u16) {
        self.state = Self::add_to_hash(self.state, i as u32);
    }
    fn write_u32(&mut self, i: u32) {
        self.state = Self::add_to_hash(self.state, i);
    }
    fn write_u64(&mut self, i: u64) {
        self.state = Self::add_to_hash(self.state, i as u32);
        self.state = Self::add_to_hash(self.state, (i >> 32) as u32);
    }
    fn write_u128(&mut self, i: u128) {
        self.state = Self::add_to_hash(self.state, i as u32);
        self.state = Self::add_to_hash(self.state, (i >> 32) as u32);
        self.state = Self::add_to_hash(self.state, (i >> 64) as u32);
        self.state = Self::add_to_hash(self.state, (i >> 96) as u32);
    }
    fn write_usize(&mut self, i: usize) {
        #[cfg(target_pointer_width = "32")]
        {
            self.write_u32(i as u32);
        }
        #[cfg(target_pointer_width = "64")]
        {
            self.write_u64(i as u64);
        }
    }
    fn finish(&self) -> u64 {
        Self::finalize(self.state) as u64
    }
}
impl Hasher for HasherFx<u64> {
    fn write(&mut self, bytes: &[u8]) {
        let compressed = hash_bytes::<true>(bytes);
        self.state = Self::add_to_hash(self.state, compressed);
    }
    fn write_u8(&mut self, i: u8) {
        self.state = Self::add_to_hash(self.state, i as u64);
    }
    fn write_u16(&mut self, i: u16) {
        self.state = Self::add_to_hash(self.state, i as u64);
    }
    fn write_u32(&mut self, i: u32) {
        self.state = Self::add_to_hash(self.state, i as u64);
    }
    fn write_u64(&mut self, i: u64) {
        self.state = Self::add_to_hash(self.state, i);
    }
    fn write_u128(&mut self, i: u128) {
        self.state = Self::add_to_hash(self.state, i as u64);
        self.state = Self::add_to_hash(self.state, (i >> 64) as u64);
    }
    fn write_usize(&mut self, i: usize) {
        self.state = Self::add_to_hash(self.state, i as u64);
    }
    fn finish(&self) -> u64 {
        Self::finalize(self.state)
    }
}
impl Hasher for HasherFx<usize> {
    fn write(&mut self, bytes: &[u8]) {
        let compressed = hash_bytes::<WIDE_MIX>(bytes);
        self.write_u64(compressed);
    }
    fn write_u8(&mut self, i: u8) {
        self.state = Self::add_to_hash(self.state, i as usize);
    }
    fn write_u16(&mut self, i: u16) {
        self.state = Self::add_to_hash(self.state, i as usize);
    }
    fn write_u32(&mut self, i: u32) {
        self.state = Self::add_to_hash(self.state, i as usize);
    }
    #[cfg(target_pointer_width = "32")]
    fn write_u64(&mut self, i: u64) {
        self.state = Self::add_to_hash(self.state, i as usize);
        self.state = Self::add_to_hash(self.state, (i >> 32) as usize);
    }
    #[cfg(target_pointer_width = "64")]
    fn write_u64(&mut self, i: u64) {
        self.state = Self::add_to_hash(self.state, i as usize);
    }
    #[cfg(target_pointer_width = "32")]
    fn write_u128(&mut self, i: u128) {
        self.state = Self::add_to_hash(self.state, i as usize);
        self.state = Self::add_to_hash(self.state, (i >> 32) as usize);
        self.state = Self::add_to_hash(self.state, (i >> 64) as usize);
        self.state = Self::add_to_hash(self.state, (i >> 96) as usize);
    }
    #[cfg(target_pointer_width = "64")]
    fn write_u128(&mut self, i: u128) {
        self.state = Self::add_to_hash(self.state, i as usize);
        self.state = Self::add_to_hash(self.state, (i >> 64) as usize);
    }
    fn write_usize(&mut self, i: usize) {
        self.state = Self::add_to_hash(self.state, i);
    }
    fn finish(&self) -> u64 {
        Self::finalize(self.state) as u64
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::Hasher;

    #[test]
    fn integer_vectors_u32() {
        assert_eq!(HasherFx::<u32>::hash(&0_u8), 0);
        assert_eq!(HasherFx::<u32>::hash(&1_u8), 3_001_993_707);
        assert_eq!(HasherFx::<u32>::hash(&100_u8), 3_844_759_569);
        assert_eq!(HasherFx::<u32>::hash(&1_u64), 275_023_839);
        assert_eq!(HasherFx::<u32>::hash(&1_u128), 1_860_738_631);
    }
    #[test]
    fn integer_vectors_u64() {
        assert_eq!(HasherFx::<u64>::hash(&0_u8), 0);
        assert_eq!(HasherFx::<u64>::hash(&1_u8), 12_157_901_119_326_311_915);
        assert_eq!(HasherFx::<u64>::hash(&100_u8), 16_751_747_135_202_103_309);
        assert_eq!(HasherFx::<u64>::hash(&1_u64), 12_157_901_119_326_311_915);
        assert_eq!(HasherFx::<u64>::hash(&1_u128), 13_032_756_267_696_824_044);
    }
    #[test]
    fn byte_vectors_u32() {
        assert_eq!(HasherFx::<u32>::hash_bytes(b""), 2_673_204_745);
        assert_eq!(HasherFx::<u32>::hash_bytes(&[0]), 2_948_228_584);
        assert_eq!(HasherFx::<u32>::hash_bytes(&[0, 0, 0, 0, 0, 0]), 3_223_252_423);
        assert_eq!(HasherFx::<u32>::hash_bytes(&[1]), 2_943_445_104);
    }
    #[test]
    fn byte_vectors_u64() {
        assert_eq!(HasherFx::<u64>::hash_bytes(b""), 17_606_491_139_363_777_937);
        assert_eq!(HasherFx::<u64>::hash_bytes(&[0]), 5_448_590_020_104_574_886);
        assert_eq!(HasherFx::<u64>::hash_bytes(&[0, 0, 0, 0, 0, 0]), 16_766_921_560_080_789_783);
        assert_eq!(HasherFx::<u64>::hash_bytes(&[1]), 5_922_447_956_811_044_110);
    }
    #[test]
    fn byte_hash_matches_hasher_write() {
        let samples: &[&[u8]] = &[
            b"",
            b"a",
            b"abcd",
            b"abcdefgh",
            b"0123456789abcdef",
            b"0123456789abcdef0",
            b"0123456789abcdef0123456789abcdef",
            b"0123456789abcdef0123456789abcdef0",
        ];
        for bytes in samples {
            let mut h32 = HasherFx::<u32>::new();
            h32.write(bytes);
            assert_eq!(HasherFx::<u32>::hash_bytes(bytes), h32.finish() as u32);
            let mut h64 = HasherFx::<u64>::new();
            h64.write(bytes);
            assert_eq!(HasherFx::<u64>::hash_bytes(bytes), h64.finish());
        }
    }
    #[test]
    fn zero_bytes_do_not_collapse() {
        assert_ne!(HasherFx::<u64>::hash_bytes(b""), HasherFx::<u64>::hash_bytes(&[0]));
        assert_ne!(HasherFx::<u64>::hash_bytes(&[0]), HasherFx::<u64>::hash_bytes(&[0; 16]));
        assert_ne!(HasherFx::<u64>::hash_bytes(&[0; 16]), HasherFx::<u64>::hash_bytes(&[0; 32]));
    }
    #[test]
    fn seeded_hashing() {
        let bytes = b"devela";
        let a = HasherFx::<u64>::hash_bytes_with_seed(1, bytes);
        let b = HasherFx::<u64>::hash_bytes_with_seed(1, bytes);
        let c = HasherFx::<u64>::hash_bytes_with_seed(2, bytes);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
    #[test]
    fn native_matches_fixed_width() {
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(
                HasherFx::<usize>::hash_bytes(b"devela"),
                HasherFx::<u32>::hash_bytes(b"devela") as usize
            );
        }
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(
                HasherFx::<usize>::hash_bytes(b"devela"),
                HasherFx::<u64>::hash_bytes(b"devela") as usize
            );
        }
    }
    #[test]
    fn primitive_bytes_match_integer_hash() {
        macro_rules! check {
            ($($v:expr),+ $(,)?) => {
                $(
                    assert_eq!(
                        HasherFx::<usize>::hash_primitive_bytes(&$v.to_le_bytes()),
                        HasherFx::<usize>::hash(&$v),
                    );
                )+
            };
        }
        check![1_u8, 1_u16, 1_u32, 1_u64, 1_u128, 1_usize,];
    }
}
