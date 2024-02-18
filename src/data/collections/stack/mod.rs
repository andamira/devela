// devela::data::collections::stack
//
//! A type that can be used as a single-ended stack.
//

/* always compiled, non-public modules */

// without re-exports
mod impl_traits;
mod methods;
// #[cfg(test)]
// mod tests;

mod definitions;
pub use definitions::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;
}
