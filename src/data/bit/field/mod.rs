// devela::data::bit::field
//
//!
//

mod bitfield;
pub use bitfield::bitfield;

#[cfg(all(test, feature = "_bit_u8"))]
mod tests;
