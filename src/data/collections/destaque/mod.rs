// devela::data::collections::destaque
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

/* modules */

// always compiled, non-public
mod definitions;

// always compiled, non-public, nothing to re-export
mod core_impls;
mod methods;
#[cfg(test)]
mod tests;

/* re-export */

// always compiled, non-public
pub use definitions::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;
}
