// devela::data::conversion::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod tests;

mod bits;
mod cast;
mod join;
mod split;

pub use {bits::*, cast::*, join::*, split::*};
