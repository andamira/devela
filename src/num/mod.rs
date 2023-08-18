// devela::num
//
//! Numerics, extends [`core::num`].
//

#[cfg(test)]
mod tests;

mod non_range;
mod non_specific;
mod range;

pub use {non_range::*, non_specific::*, range::*};
