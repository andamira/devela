// devela::num::prob::rand::_helper
//
//! Defines (`__impl_dep_rand_core!`)
//

/// No-op fallback for `rand_core` adapter implementations when the dependency is disabled.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "dep_rand_core"))]
macro_rules! __impl_dep_rand_core {
    ($rng:ident $(<$(const $C:ident: $T:ty),+ $(,)?>)?) => {};
}

/// Implements `rand_core` adapter traits for a possibly const-generic RNG.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "dep_rand_core")]
macro_rules! __impl_dep_rand_core {
    ($rng:ident $(<$(const $C:ident: $T:ty),+ $(,)?>)?) => {
        $crate::_devela_policy! {
            item_attr devela { #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))] }
            impl $(<$(const $C: $T),+>)? $crate::_dep::rand_core::TryRng
                for $rng $(<$($C),+>)? {

                type Error = <Self as $crate::RandTry>::Error;

                #[inline(always)]
                fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
                    $crate::RandTry::rand_try_next_u32(self)
                }
                #[inline(always)]
                fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
                    $crate::RandTry::rand_try_next_u64(self)
                }
                #[inline(always)]
                fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
                    $crate::RandTry::rand_try_fill_bytes(self, dst)
                }
            }
        }
        $crate::_devela_policy! {
            item_attr devela { #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_rand_core")))] }
            impl $(<$(const $C: $T),+>)? $crate::_dep::rand_core::SeedableRng
                for $rng $(<$($C),+>)? {

                type Seed = <Self as $crate::RandSeedable>::RandSeed;

                #[inline(always)]
                fn from_seed(seed: Self::Seed) -> Self {
                    $crate::RandSeedable::rand_from_seed(seed)
                }
            }
        }
    };
}
#[doc(hidden)]
pub use __impl_dep_rand_core;
