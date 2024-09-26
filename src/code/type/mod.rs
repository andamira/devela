// devela::code::type
//
//! Type safety.
//

mod marker; // zero-cost generic type markers
mod res; // zero-cost type-safe resource markers
#[allow(unused_imports)]
pub use {marker::*, res::*};
