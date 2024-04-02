// devela::data::collections::destaque
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

/* always compiled */

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
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;
}
