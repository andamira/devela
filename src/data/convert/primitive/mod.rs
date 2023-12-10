// devela::data::conversion::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod tests;

mod bitwise;
mod cast;
mod join;
mod split;

pub use {bitwise::*, cast::*, join::*, split::*};
