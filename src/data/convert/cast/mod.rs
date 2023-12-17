// devela::data::conversion::cast
//
//! Helpers for converting between cast.
//

#[cfg(test)]
mod tests;

mod casts;
mod join;
mod split;

pub use {casts::*, join::*, split::*};

/// Provides constant casting methods for `T`.
///
/// It implements the same methods as the [`CastPrimitives`], [`FromPrimitives`],
/// and [`IntoPrimitives`] traits.
#[repr(transparent)]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub struct Casting<T>(pub T);
