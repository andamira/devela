// devela::data::collections::array::d2
//
//! 2-dimensional array
//

// without re-exports
mod impl_traits;
mod methods;
#[cfg(test)]
mod tests;
// with re-exports
mod definitions;
#[allow(unused_imports)]
pub use definitions::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::definitions::*;
}
