// devela::data::hash::fnv
//
//! Fowler–Noll–Vo hash function
//!
//! - <https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function>
//! - <http://www.isthe.com/chongo/tech/comp/fnv>
//

use crate::{
    _libcore::{concat as cc, stringify as fy},
    code::ConstDefault,
    num::PrimitiveCast,
};

/// A Fowler–Noll–Vo hasher.
///
/// It's implemented for
/// [u32](#impl-HasherFnv<u32>),
/// [u64](#impl-HasherFnv<u64>),
/// [u128](#impl-HasherFnv<u128>).
///
/// It uses the `fnv-1a` variation which gives better avalanche characteristics.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HasherFnv<T> {
    state: T,
}

macro_rules! impl_fnv {
    () => {
        impl_fnv![
            u32:
                0x811c9dc5:
                0x1000193,
            u64:
                0x00000100000001B3:
                0xcbf29ce484222325,
            u128:
                0x0000000001000000000000000000013B:
                0x6c62272e07bb014262b821756295c58d
        ];
    };

    ($($t:ty:$basis:literal:$prime:literal),+) =>  { $( impl_fnv![@$t:$basis:$prime]; )+ };
    (@$t:ty:$basis:literal:$prime:literal) =>  {

        impl ConstDefault for HasherFnv<$t> { const DEFAULT: Self = Self { state: $basis }; }
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
                    self.state = self.state.wrapping_mul($prime);
                }
            }

            /// Resets the inner state to the default basis value.
            #[inline]
            pub fn reset(&mut self) {
                self.state = $basis;
            }

            /* state-less methods */

            /// Computes the FNV hash of the provided byte slice.
            #[inline] #[must_use]
            pub const fn hash(input: &[u8]) -> $t {
                let mut hash = $basis;
                let mut i = 0;
                while i < input.len() {
                    hash ^= input[i] as $t;
                    hash = hash.wrapping_mul($prime);
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
                    hash = (hash.wrapping_mul($prime)).wrapping_add($basis);
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
                    hash = (hash.wrapping_mul($prime)).wrapping_add($basis);
                }
                hash % range
            }
        }

        impl core::hash::Hasher for HasherFnv<$t> {
            #[inline]
            fn write(&mut self, bytes: &[u8]) {
                for byte in bytes {
                    self.state ^= <$t>::from(*byte);
                    self.state = self.state.wrapping_mul($prime);
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
