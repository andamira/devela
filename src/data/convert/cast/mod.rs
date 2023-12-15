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
pub struct Cast<T>(pub T);
