// devela::data::collections::stack
//
//! A type that can be used as a single-ended stack.
//

/* modules */

// always compiled, non-public
mod definitions;

// always compiled, non-public, nothing to re-export
mod impl_traits;
mod methods;
mod methods_own;
// #[cfg(test)]
// mod tests;

/* re-export */

// always compiled, non-public
pub use definitions::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;
}
