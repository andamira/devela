// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(test)]
mod tests;
//
#[cfg(_some_in_range)]
mod in_range;
#[cfg(_some_non_range)]
mod non_range;
mod non_value;
#[cfg(_some_in_range)]
pub use in_range::*;
#[cfg(_some_non_range)]
pub use non_range::*;
pub use non_value::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(_some_in_range)]
    pub use super::in_range::*;
    #[doc(inline)]
    #[cfg(_some_non_range)]
    pub use super::non_range::*;
    #[doc(inline)]
    pub use super::non_value::*;
}
