// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(_niche_some)]
#[cfg(test)]
mod tests;
//
#[cfg(_in_range_some)]
mod in_range;
#[cfg(_non_range_some)]
mod non_range;
#[cfg(_non_value_some)]
mod non_value;
#[cfg(_in_range_some)]
pub use in_range::*;
#[cfg(_non_range_some)]
pub use non_range::*;
#[cfg(_non_value_some)]
pub use non_value::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(_in_range_some)]
    pub use super::in_range::*;
    #[doc(inline)]
    #[cfg(_non_range_some)]
    pub use super::non_range::*;
    #[doc(inline)]
    #[cfg(_non_value_some)]
    pub use super::non_value::*;
}
