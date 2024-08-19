// devela::data::hash::fx
//
//! # Fx Hash
//

use core::hash::{BuildHasherDefault, Hash, Hasher};

/// A builder for default Fx hashers.
pub type HasherBuildFx = BuildHasherDefault<HasherFx>;

const DEFAULT32: u32 = 0;
const DEFAULT64: u64 = 0;
const DEFAULT: usize = 0;

const ROTATE: u32 = 5;
const SEED64: u64 = 0x51_7c_c1_b7_27_22_0a_95;
const SEED32: u32 = 0x9e_37_79_b9;

#[cfg(target_pointer_width = "32")]
const SEED: usize = SEED32 as usize;
#[cfg(target_pointer_width = "64")]
const SEED: usize = SEED64 as usize;

trait HashWord {
    fn hash_word(&mut self, word: Self);
}
impl HashWord for u32 {
    #[inline]
    fn hash_word(&mut self, word: Self) {
        *self = HasherFx32::hash_word_32(*self, word);
    }
}
impl HashWord for u64 {
    #[inline]
    fn hash_word(&mut self, word: Self) {
        *self = HasherFx64::hash_word_64(*self, word);
    }
}
impl HashWord for usize {
    #[inline]
    fn hash_word(&mut self, word: Self) {
        *self = HasherFx::hash_word(*self, word);
    }
}

/// This hashing algorithm was extracted from the Rustc compiler.
///
/// This is the same hashing algorithm used for some internal operations in
/// Firefox. The strength of this algorithm is in hashing 4 bytes at a time on
/// any platform, where the FNV algorithm works on one byte at a time.
///
/// This hashing algorithm should not be used for cryptographic,
/// or in scenarios where DOS attacks are a concern.
#[derive(Clone, Debug)]
pub struct HasherFx32 {
    hash: u32,
}

impl Default for HasherFx32 {
    #[inline]
    fn default() -> HasherFx32 {
        Self::default()
    }
}

impl HasherFx32 {
    /// New 32-bit sized Fx hasher.
    #[inline]
    pub const fn default() -> Self {
        Self { hash: DEFAULT32 }
    }

    /// New 32-bit sized Fx hasher with `seed`.
    #[inline]
    pub const fn with_seed(seed: u32) -> HasherFx32 {
        HasherFx32 { hash: seed }
    }

    /// A convenience method for when you need a quick 32-bit hash.
    #[inline]
    pub fn hash32<T: Hash + ?Sized>(v: &T) -> u32 {
        let mut state = HasherFx32::default();
        v.hash(&mut state);
        state.finish() as u32
    }

    /// A const method for when you need a 32-bit hash of a byte array.
    #[inline]
    pub const fn hash32_bytes(v: &[u8]) -> u32 {
        Self::write32(0, v)
    }

    #[inline]
    const fn hash_word_32(mut hash: u32, word: u32) -> u32 {
        hash = hash.rotate_left(ROTATE);
        hash ^= word;
        hash = hash.wrapping_mul(SEED32);
        hash
    }

    #[inline]
    const fn write32(mut hash: u32, bytes: &[u8]) -> u32 {
        let mut cursor = 0;

        while bytes.len() - cursor >= 4 {
            let word = u32::from_ne_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
            ]);
            hash = Self::hash_word_32(hash, word);
            cursor += 4;
        }

        if bytes.len() - cursor >= 2 {
            let word = u16::from_ne_bytes([bytes[cursor], bytes[cursor + 1]]);
            hash = Self::hash_word_32(hash, word as u32);
            cursor += 2;
        }

        if bytes.len() - cursor >= 1 {
            hash = Self::hash_word_32(hash, bytes[cursor] as u32);
        }

        hash
    }
}

impl Hasher for HasherFx32 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = Self::write32(self.hash, bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(u32::from(i));
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(u32::from(i));
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash.hash_word(i);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash.hash_word(i as u32);
        self.hash.hash_word((i >> 32) as u32);
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn write_usize(&mut self, i: usize) {
        self.write_u32(i as u32);
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        u64::from(self.hash)
    }
}

/// This hashing algorithm was extracted from the Rustc compiler.
///
/// This is the same hashing algorithm used for some internal operations in
/// Firefox. The strength of this algorithm is in hashing 8 bytes at a time on
/// any platform, where the FNV algorithm works on one byte at a time.
///
/// This hashing algorithm should not be used for cryptographic,
/// or in scenarios where DOS attacks are a concern.
#[derive(Clone, Debug)]
pub struct HasherFx64 {
    hash: u64,
}

impl Default for HasherFx64 {
    #[inline]
    fn default() -> HasherFx64 {
        Self::default()
    }
}

impl HasherFx64 {
    /// New 64-bit sized Fx hasher.
    #[inline]
    pub const fn default() -> Self {
        Self { hash: DEFAULT64 }
    }

    /// New 64-bit sized Fx hasher with `seed`.
    #[inline]
    pub const fn with_seed(seed: u64) -> HasherFx64 {
        HasherFx64 { hash: seed }
    }

    /// A convenience method for when you need a quick 64-bit hash.
    #[inline]
    pub fn hash64<T: Hash + ?Sized>(v: &T) -> u64 {
        let mut state = HasherFx64::default();
        v.hash(&mut state);
        state.finish()
    }

    /// A const method for when you need a 64-bit hash of a byte array.
    #[inline]
    pub const fn hash64_bytes(v: &[u8]) -> u64 {
        Self::write64(0, v)
    }

    #[inline]
    const fn hash_word_64(mut hash: u64, word: u64) -> u64 {
        hash = hash.rotate_left(ROTATE);
        hash ^= word;
        hash = hash.wrapping_mul(SEED64);
        hash
    }

    #[inline]
    const fn write64(mut hash: u64, bytes: &[u8]) -> u64 {
        let mut cursor = 0;

        while bytes.len() - cursor >= 8 {
            let word = u64::from_ne_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
                bytes[cursor + 4],
                bytes[cursor + 5],
                bytes[cursor + 6],
                bytes[cursor + 7],
            ]);
            hash = Self::hash_word_64(hash, word);
            cursor += 8;
        }

        while bytes.len() - cursor >= 4 {
            let word = u32::from_ne_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
            ]);
            hash = Self::hash_word_64(hash, word as u64);
            cursor += 4;
        }

        if bytes.len() - cursor >= 2 {
            let word = u16::from_ne_bytes([bytes[cursor], bytes[cursor + 1]]);
            hash = Self::hash_word_64(hash, word as u64);
            cursor += 2;
        }

        if bytes.len() - cursor >= 1 {
            hash = Self::hash_word_64(hash, bytes[cursor] as u64);
        }

        hash
    }
}

impl Hasher for HasherFx64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = Self::write64(self.hash, bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(u64::from(i));
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(u64::from(i));
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash.hash_word(u64::from(i));
    }

    fn write_u64(&mut self, i: u64) {
        self.hash.hash_word(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash.hash_word(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

/// This hashing algorithm was extracted from the Rustc compiler.
///
/// This is the same hashing algorithm used for some internal operations in Firefox.
/// The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms,
/// where the FNV algorithm works on one byte at a time.
///
/// This hashing algorithm should not be used for cryptographic, or in scenarios where
/// DOS attacks are a concern.
#[derive(Clone, Debug)]
pub struct HasherFx {
    hash: usize,
}

impl Default for HasherFx {
    #[inline]
    fn default() -> HasherFx {
        Self::default()
    }
}

impl HasherFx {
    /// New `usize` sized Fx hasher.
    #[inline]
    pub const fn default() -> Self {
        Self { hash: DEFAULT }
    }

    /// New `usize` sized Fx hasher with `seed`.
    #[inline]
    pub const fn with_seed(seed: usize) -> HasherFx {
        HasherFx { hash: seed }
    }

    /// A convenience method for when you need a quick `usize` hash.
    #[inline]
    pub fn hash<T: Hash + ?Sized>(v: &T) -> usize {
        let mut state = HasherFx::default();
        v.hash(&mut state);
        state.finish() as usize
    }

    /// A const method for when you need a usize hash of a byte array.
    #[inline]
    pub const fn hash_bytes(v: &[u8]) -> usize {
        Self::write(0, v)
    }

    #[inline]
    const fn hash_word(mut hash: usize, word: usize) -> usize {
        hash = hash.rotate_left(ROTATE);
        hash ^= word;
        hash = hash.wrapping_mul(SEED);
        hash
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    const fn write(hash: usize, bytes: &[u8]) -> usize {
        HasherFx32::write32(hash as u32, bytes) as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    const fn write(hash: usize, bytes: &[u8]) -> usize {
        HasherFx64::write64(hash as u64, bytes) as usize
    }
}

impl Hasher for HasherFx {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = Self::write(self.hash, bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(i as usize);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(i as usize);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash.hash_word(i as usize);
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn write_u64(&mut self, i: u64) {
        self.hash.hash_word(i as usize);
        self.hash.hash_word((i >> 32) as usize);
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn write_u64(&mut self, i: u64) {
        self.hash.hash_word(i as usize);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash.hash_word(i);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}
