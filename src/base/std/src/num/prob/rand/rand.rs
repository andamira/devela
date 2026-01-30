// devela_base_std::num::prob::rand::rand
//
//! Defines [`RandStd`].
//
// TODO: implement RandStd for the rest of PRNGs

use crate::{Hasher, HasherBuild, Rand, RandomState};

/// Randomness extensions that require the standard library.
pub trait RandStd: Rand {
    /// Seeds a new generator using operating system entropy.
    ///
    /// The quality depends on the platform and is not guaranteed to be
    /// cryptographically secure.
    fn rand_seed_from_os() -> Self;
}

/* impls */

impl RandStd for crate::Pcg32 {
    /// Creates a new `Pcg32` PRNG, seeded from [`RandomState`].
    #[inline(never)]
    fn rand_seed_from_os() -> Self {
        let h = RandomState::new();
        let (mut hasher1, mut hasher2) = (h.build_hasher(), h.build_hasher());
        hasher1.write_u64(Self::DEFAULT_SEED);
        hasher2.write_u64(Self::DEFAULT_INC);
        let (state, inc) = (hasher1.finish(), hasher2.finish() | 1);
        Self::new_unchecked(state, inc)
    }
}
impl RandStd for crate::XorShift128p {
    /// Creates a new `XoroShift128p` PRNG, seeded from [`RandomState`].
    #[inline(never)]
    fn rand_seed_from_os() -> Self {
        let h = RandomState::new();
        let (mut hasher1, mut hasher2) = (h.build_hasher(), h.build_hasher());
        hasher1.write_u64(Self::DEFAULT_SEED[0]);
        hasher2.write_u64(Self::DEFAULT_SEED[0]);
        let seed = [hasher1.finish(), hasher2.finish()];
        Self::new_unchecked(seed)
    }
}

#[cfg(feature = "rand")]
impl RandStd for crate::Xoroshiro128pp {
    /// Creates a new `Xoroshiro128++` PRNG, seeded from [`RandomState`].
    #[inline(never)]
    #[cfg(feature = "std")]
    fn rand_seed_from_os() -> Self {
        let h = RandomState::new();
        let (mut hasher1, mut hasher2) = (h.build_hasher(), h.build_hasher());
        hasher1.write_u32(Self::DEFAULT_SEED[0]);
        hasher2.write_u32(Self::DEFAULT_SEED[0]);
        let (hash1, hash2) = (hasher1.finish(), hasher2.finish());
        let seed = [(hash1 >> 32) as u32, hash1 as u32, (hash2 >> 32) as u32, hash2 as u32];
        Self::new_unchecked(seed)
    }
}
