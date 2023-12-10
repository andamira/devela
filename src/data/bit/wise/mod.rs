// devela::data::bit::wise
//
//! bitwise manipulation helpers
//

mod r#trait;
mod wrapper;
pub use {r#trait::Bitwise, wrapper::Bits};

#[cfg(test)]
mod tests;
