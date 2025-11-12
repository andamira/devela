// devela::data::codec::hash::fx

use crate::{ConstInit, Hash, Hasher, HasherBuildDefault};

/// A builder for default Fx hashers.
pub type HasherBuildFx = HasherBuildDefault<HasherFx<usize>>;

/// A hashing algorithm used in the Rustc compiler, implemented for
/// [u32](#impl-HasherFx<u32>),
/// [u64](#impl-HasherFx<u64>) &
/// [usize](##impl-HasherFx<usize>).
///
/// This is the same hashing algorithm used for some internal operations in
/// Firefox. The strength of this algorithm is in hashing 4 bytes at a time on
/// any platform, where the FNV algorithm works on one byte at a time.
///
/// This hashing algorithm should not be used for cryptographic,
/// or in scenarios where DoS attacks are a concern.
///
#[doc = crate::_doc!(vendor: "fxhash")]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HasherFx<T> {
    state: T,
}

const ROTATE: u32 = 5;
const SEED32: u32 = 0x9e_37_79_b9;
const SEED64: u64 = 0x51_7c_c1_b7_27_22_0a_95;
#[cfg(target_pointer_width = "32")]
const SEED: usize = SEED32 as usize;
#[cfg(target_pointer_width = "64")]
const SEED: usize = SEED64 as usize;

macro_rules! impl_fx {
    () => { impl_fx![u32:SEED32, u64:SEED64, usize:SEED]; };

    ($($t:ty:$seed:ident),+) =>  { $( impl_fx![@$t:$seed]; )+ };
    (@$t:ty:$seed:ident) =>  {
        impl ConstInit for HasherFx<$t> { const INIT: Self = Self { state: 0 }; }
        impl Default for HasherFx<$t> { fn default() -> Self { Self::INIT } }

        impl HasherFx<$t> {
            /* state-full methods */

            /// Returns a default Fx hasher.
            pub const fn new() -> Self { Self::INIT }

            /// Returns a new Fx hasher with the given `seed`.
            pub const fn with_seed(seed: $t) -> Self { Self { state: seed } }

            /// A convenience method for when you need a quick hash.
            pub fn hash<T: Hash + ?Sized>(v: &T) -> $t {
                let mut state = Self::new();
                v.hash(&mut state);
                state.state
            }

            /// Computes a hash of a byte slice using the **default seed**.
            ///
            /// Equivalent to calling [`Self::hash_bytes_with_seed`] with `0`.
            pub const fn hash_bytes(v: &[u8]) -> $t {
                Self::hash_bytes_with_seed(0, v)
            }

            /// Computes a hash of a byte slice using a **custom seed**.
            ///
            /// This allows deterministic but varied hashing for different use cases.
            pub const fn hash_bytes_with_seed(seed: $t, v: &[u8]) -> $t {
                Self::write(seed, v)
            }

            const fn hash_word(mut hash: $t, word: $t) -> $t {
                hash = hash.rotate_left(ROTATE);
                hash ^= word;
                hash = hash.wrapping_mul($seed);
                hash
            }
        }
    };
}
impl_fx!();

/* impl HasherFx::write */

impl HasherFx<u32> {
    const fn write(mut hash: u32, bytes: &[u8]) -> u32 {
        let mut cursor = 0;
        while bytes.len() - cursor >= 4 {
            let word = u32::from_ne_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
            ]);
            hash = Self::hash_word(hash, word);
            cursor += 4;
        }
        if bytes.len() - cursor >= 2 {
            let word = u16::from_ne_bytes([bytes[cursor], bytes[cursor + 1]]);
            hash = Self::hash_word(hash, word as u32);
            cursor += 2;
        }
        if bytes.len() - cursor >= 1 {
            hash = Self::hash_word(hash, bytes[cursor] as u32);
        }
        hash
    }
}

impl HasherFx<u64> {
    const fn write(mut hash: u64, bytes: &[u8]) -> u64 {
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
            hash = Self::hash_word(hash, word);
            cursor += 8;
        }
        while bytes.len() - cursor >= 4 {
            let word = u32::from_ne_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
            ]);
            hash = Self::hash_word(hash, word as u64);
            cursor += 4;
        }
        if bytes.len() - cursor >= 2 {
            let word = u16::from_ne_bytes([bytes[cursor], bytes[cursor + 1]]);
            hash = Self::hash_word(hash, word as u64);
            cursor += 2;
        }
        if bytes.len() - cursor >= 1 {
            hash = Self::hash_word(hash, bytes[cursor] as u64);
        }
        hash
    }
}

impl HasherFx<usize> {
    const fn write(hash: usize, bytes: &[u8]) -> usize {
        #[cfg(target_pointer_width = "32")]
        return HasherFx::<u32>::write(hash as u32, bytes) as usize;
        #[cfg(target_pointer_width = "64")]
        return HasherFx::<u64>::write(hash as u64, bytes) as usize;
    }
}

/* impl Hasher for HasherFx */

impl Hasher for HasherFx<u32> {
    fn write(&mut self, bytes: &[u8]) {
        self.state = Self::write(self.state, bytes);
    }
    fn write_u8(&mut self, i: u8) {
        self.state = Self::hash_word(self.state, u32::from(i));
    }
    fn write_u16(&mut self, i: u16) {
        self.state = Self::hash_word(self.state, u32::from(i));
    }
    fn write_u32(&mut self, i: u32) {
        self.state = Self::hash_word(self.state, i);
    }
    fn write_u64(&mut self, i: u64) {
        self.state = Self::hash_word(self.state, i as u32);
        self.state = Self::hash_word(self.state, (i >> 32) as u32);
    }
    fn write_usize(&mut self, i: usize) {
        #[cfg(target_pointer_width = "32")]
        self.write_u32(i as u32);
        #[cfg(target_pointer_width = "64")]
        self.write_u64(i as u64);
    }
    fn finish(&self) -> u64 {
        self.state as u64
    }
}
impl Hasher for HasherFx<u64> {
    fn write(&mut self, bytes: &[u8]) {
        self.state = Self::write(self.state, bytes);
    }
    fn write_u8(&mut self, i: u8) {
        self.state = Self::hash_word(self.state, u64::from(i));
    }
    fn write_u16(&mut self, i: u16) {
        self.state = Self::hash_word(self.state, u64::from(i));
    }
    fn write_u32(&mut self, i: u32) {
        self.state = Self::hash_word(self.state, u64::from(i));
    }
    fn write_u64(&mut self, i: u64) {
        self.state = Self::hash_word(self.state, i);
    }
    fn write_usize(&mut self, i: usize) {
        self.state = Self::hash_word(self.state, i as u64);
    }
    fn finish(&self) -> u64 {
        self.state
    }
}
impl Hasher for HasherFx<usize> {
    fn write(&mut self, bytes: &[u8]) {
        self.state = Self::write(self.state, bytes);
    }
    fn write_u8(&mut self, i: u8) {
        self.state = Self::hash_word(self.state, i as usize);
    }
    fn write_u16(&mut self, i: u16) {
        self.state = Self::hash_word(self.state, i as usize);
    }
    fn write_u32(&mut self, i: u32) {
        self.state = Self::hash_word(self.state, i as usize);
    }
    #[cfg(target_pointer_width = "32")]
    fn write_u64(&mut self, i: u64) {
        self.state = Self::hash_word(self.state, i as usize);
        self.state = Self::hash_word(self.state, (i >> 32) as usize);
    }
    #[cfg(target_pointer_width = "64")]
    fn write_u64(&mut self, i: u64) {
        self.state = Self::hash_word(self.state, i as usize);
    }
    fn write_usize(&mut self, i: usize) {
        self.state = HasherFx::<usize>::hash_word(self.state, i);
    }
    fn finish(&self) -> u64 {
        self.state as u64
    }
}
