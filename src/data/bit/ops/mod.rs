// devela::data::bit::ops
//
//! bitwise manipulation helpers
//

mod r#trait;
mod wrapper;
pub use {r#trait::BitOps, wrapper::Bits};

#[cfg(test)]
mod tests;
