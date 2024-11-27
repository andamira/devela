// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

#[cfg(test)]
mod tests;

mod non_value;
mod reexports;
#[allow(unused_imports)]
pub use {non_value::*, reexports::*};

pub(crate) mod all {
    #![allow(unused_imports)]
    #[doc(inline)]
    pub use super::{non_value::*, reexports::*};
}
