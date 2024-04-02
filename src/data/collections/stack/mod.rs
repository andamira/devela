// devela::data::collections::stack
//
//! A type that can be used as a single-ended stack.
//

/* always compiled */

// without re-exports
mod impl_traits;
mod methods;
// with re-exports
mod definitions;
#[allow(unused_imports)]
pub use definitions::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;
}
