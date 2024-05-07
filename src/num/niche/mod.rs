// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(feature = "_-niche_any-_")]
#[cfg(test)]
mod tests;
//
#[cfg(feature = "_-in_range_any-_")]
mod in_range;
#[cfg(feature = "_-non_range_any-_")]
mod non_range;
#[cfg(feature = "_-non_value_any-_")]
mod non_value;
#[cfg(feature = "_-in_range_any-_")]
pub use in_range::*;
#[cfg(feature = "_-non_range_any-_")]
pub use non_range::*;
#[cfg(feature = "_-non_value_any-_")]
pub use non_value::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(feature = "_-in_range_any-_")]
    pub use super::in_range::*;
    #[doc(inline)]
    #[cfg(feature = "_-non_range_any-_")]
    pub use super::non_range::*;
    #[doc(inline)]
    #[cfg(feature = "_-non_value_any-_")]
    pub use super::non_value::*;
}
