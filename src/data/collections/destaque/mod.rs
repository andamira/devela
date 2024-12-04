// devela::data::collections::destaque
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

// no items defined
mod impl_traits;
mod methods;
#[cfg(all(test, feature = "_destaque_u8"))]
mod tests;

mod definitions;

/* structural access */

#[allow(unused_imports)]
pub use doc_inline::*;

mod doc_inline {
    pub use super::definitions::*;
}
pub(super) mod all {
    #[doc(inline)]
    pub use super::doc_inline::*;
}
