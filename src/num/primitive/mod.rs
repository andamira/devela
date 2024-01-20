// devela::num::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod tests;

mod casts;
mod join;
mod split;

pub use {casts::*, join::*, split::*};

/// Provides *const* casting, joining and splitting operations between primitives.
///
/// See also the related traits: [`PrimitiveCast`], [`PrimitiveJoin`] and [`PrimitiveSplit`].
#[repr(transparent)]
pub struct Primiting<T>(pub T);
