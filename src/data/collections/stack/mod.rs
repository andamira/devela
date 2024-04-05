// devela::data::collections::stack
//
//! A type that can be used as a single-ended stack.
//

mod impl_traits;
mod methods;

mod definitions;
#[allow(unused_imports)]
pub use definitions::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::definitions::*;
}
