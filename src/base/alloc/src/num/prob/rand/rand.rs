// devela_base_alloc::num::prob::rand::rand
//
//! Defines [`RandAlloc`].
//
// TODO: implement RandAlloc for the rest of PRNGs

use crate::{Box, Rand};

/// Randomness extensions that require allocation.
pub trait RandAlloc: Rand {
    /// Seeds a new generator using heap-related addresses.
    ///
    /// This is a very weak source of entropy.
    fn rand_seed_from_heap() -> Self;
}

/* impls */

impl RandAlloc for crate::Pcg32 {
    /// Creates a new `Pcg32` PRNG, seeded from addresses of heap and stack variables.
    #[inline(never)]
    fn rand_seed_from_heap() -> Self {
        let (a, b) = (0, Box::new(0));
        let (state, inc): (u64, u64) = (&a as *const _ as u64, &b as *const _ as u64 | 1);
        Self::new_unchecked(state, inc)
    }
}
impl RandAlloc for crate::XorShift128p {
    /// Creates a new `XoroShift128p` PRNG, seeded from addresses of heap and stack variables.
    #[inline(never)]
    fn rand_seed_from_heap() -> Self {
        let (a, b) = (0, Box::new(0));
        let seed: [u64; 2] = [&a as *const _ as u64, &b as *const _ as u64];
        Self::new_unchecked(seed)
    }
}

#[cfg(feature = "rand")]
impl RandAlloc for crate::Xoroshiro128pp {
    /// Creates a new `Xoroshiro128++` PRNG, seeded from addresses of heap and stack variables.
    #[inline(never)]
    fn rand_seed_from_heap() -> Self {
        let (a, b, c, d) = (0, Box::new(0), Box::new(0), 0);
        let seed: [u32; 4] = [
            &a as *const _ as u32,
            &b as *const _ as u32,
            &c as *const _ as u32,
            &d as *const _ as u32,
        ];
        Self::new_unchecked(seed)
    }
}
