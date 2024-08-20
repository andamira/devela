// devela::data::hash::fnv

use crate::{
    _dep::_libcore::{
        concat as cc,
        hash::{BuildHasherDefault, Hasher},
        stringify as fy,
    },
    code::ConstDefault,
    num::PrimitiveCast,
};

/// A builder for default Fnv hashers.
pub type HasherBuildFnv = BuildHasherDefault<HasherFnv<usize>>;

/// A Fowler–Noll–Vo hasher.
///
/// It's implemented for
/// [u32](#impl-HasherFnv<u32>),
/// [u64](#impl-HasherFnv<u64>),
/// [u128](#impl-HasherFnv<u128>) and
/// [usize](#impl-HasherFnv<usize>).
///
/// It uses the `fnv-1a` variation which gives better avalanche characteristics.
///
/// See
/// - <https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function>
/// - <http://www.isthe.com/chongo/tech/comp/fnv>
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HasherFnv<T> {
    state: T,
}

const BASIS32: u32 = 0x811c_9dc5;
const PRIME32: u32 = 0x0100_0193;
const BASIS64: u64 = 0x0000_0100_0000_01B3;
const PRIME64: u64 = 0xcbf2_9ce4_8422_2325;
const BASIS128: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013B;
const PRIME128: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;

macro_rules! impl_fnv {
    () => {
        impl_fnv![u32:BASIS32:PRIME32, u64:BASIS64:PRIME64, u128:BASIS128:PRIME128];
        #[cfg(target_pointer_width = "32")]
        impl_fnv![usize:BASIS32:PRIME32];
        #[cfg(target_pointer_width = "64")]
        impl_fnv![usize:BASIS64:PRIME64];
    };

    ($($t:ty:$basis:ident:$prime:ident),+) =>  { $( impl_fnv![@$t:$basis:$prime]; )+ };
    (@$t:ty:$basis:ident:$prime:ident) =>  {

        impl ConstDefault for HasherFnv<$t> { const DEFAULT: Self = Self { state: $basis as $t }; }
        impl Default for HasherFnv<$t> { #[inline] fn default() -> Self { Self::DEFAULT } }

        impl HasherFnv<$t> {
            /* state-full methods */

            /// Returns a default FNV hasher.
            #[inline]
            pub const fn new() -> Self { Self::DEFAULT }

            /// Returns an FNV hasher with the given `input` data.
            #[inline]
            pub const fn with(input: &[u8]) -> Self { Self { state: Self::hash(input) } }

            /// Returns the current hash value.
            #[inline] #[must_use]
            pub const fn get(&self) -> $t { self.state }

            /// Returns the hash value with lazy mod mapping to the given `range`.
            #[inline] #[must_use]
            pub const fn get_hash_mod_lazy(&self, range: $t) -> $t {
                self.state % range
            }

            /// Returns the hash value with retried mod mapping to the given `range`.
            #[inline] #[must_use]
            pub const fn get_hash_mod_retry(&self, range: $t) -> $t {
                Self::mod_retry_hash(self.state, range)
            }

            /// Returns the hash value xor folded to `n` bits.
            ///
            /// # Panics
            #[doc =  cc!["Panics in debug if `n` exceeds [`", fy![$t], "::BITS`]."]]
            #[inline] #[must_use]
            pub const fn get_hash_n_bits(&self, n: usize) -> $t {
                Self::fold_hash(self.state, n)
            }

            /// Updates the hasher with more data.
            ///
            /// Allows the hasher to receive additional bytes incrementally.
            #[inline]
            pub fn update(&mut self, input: &[u8]) {
                for &byte in input {
                    self.state ^= <$t>::from(byte);
                    self.state = self.state.wrapping_mul($prime as $t);
                }
            }

            /// Resets the inner state to the default basis value.
            #[inline]
            pub fn reset(&mut self) {
                self.state = $basis as $t;
            }

            /* state-less methods */

            /// Computes the FNV hash of the provided byte slice.
            #[inline] #[must_use]
            pub const fn hash(input: &[u8]) -> $t {
                let mut hash = $basis as $t;
                let mut i = 0;
                while i < input.len() {
                    hash ^= input[i] as $t;
                    hash = hash.wrapping_mul($prime as $t);
                    i += 1;
                }
                hash
            }

            /// Maps the computed FNV hash to the given `range` using lazy mod mapping.
            ///
            /// This method only does an additional mod at the end.
            /// But there's a small bias against the larger values.
            #[inline] #[must_use]
            pub fn hash_mod_lazy(input: &[u8], range: $t) -> $t {
                Self::hash(input) % range
            }

            /// Maps the computed FNV hash to the given `range` using retried mod mapping.
            #[inline] #[must_use]
            pub fn hash_mod_retry(input: &[u8], range: $t) -> $t {
                let mut hash = Self::hash(input);
                let retry_level = (<$t>::MAX / range) * range;
                while hash >= retry_level {
                    hash = (hash.wrapping_mul($prime as $t)).wrapping_add($basis as $t);
                }
                hash % range
            }

            /// Computes the FNV hash of the provided byte slice, xor folded to `n` bits.
            ///
            /// # Panics
            #[doc =  cc!["Panics in debug if `n` exceeds [`", fy![$t], "::BITS`]."]]
            #[inline] #[must_use]
            pub fn hash_n_bits(input: &[u8], n: usize) -> $t {
                Self::fold_hash(Self::hash(input), n)
            }

            /* helper methods */

            /// Reduces a hash to `n` bits using xor folding.
            ///
            /// # Panics
            #[doc =  cc!["Panics in debug if `n` exceeds [`", fy![$t], "::BITS`]."]]
            #[inline] #[must_use]
            pub const fn fold_hash(mut hash: $t, n: usize) -> $t {
                debug_assert![n <= <$t>::BITS as usize];
                let full_bits = <$t>::BITS as usize;
                let mask = (1 << n) - 1;
                // if n < full_bits { // MAYBE an alterantive to panicking
                let right_shifted = hash >> (full_bits - n);
                hash ^= right_shifted;
                hash & mask
            }

            /// Maps a hash to the given `range` using retried mod mapping.
            ///
            /// Ensures that the hash value is uniform and unbiased within the range.
            #[inline] #[must_use]
            pub const fn mod_retry_hash(mut hash: $t, range: $t) -> $t {
                let retry_level = (<$t>::MAX / range) * range;
                while hash >= retry_level {
                    hash = (hash.wrapping_mul($prime as $t)).wrapping_add($basis as $t);
                }
                hash % range
            }
        }

        impl Hasher for HasherFnv<$t> {
            #[inline]
            fn write(&mut self, bytes: &[u8]) {
                for byte in bytes {
                    self.state ^= <$t>::from(*byte);
                    self.state = self.state.wrapping_mul($prime as $t);
                }
            }

            #[inline]
            fn finish(&self) -> u64 {
                self.state.wrapping_cast_to_u64()
            }
        }
    };
}
impl_fnv!();
