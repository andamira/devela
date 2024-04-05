// devela::data::collections::destaque
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

// without re-exports
mod impl_traits;
mod methods;
#[cfg(all(test, feature = "_destaque_u8"))]
mod tests;

// with re-exports
mod definitions;
#[allow(unused_imports)]
pub use definitions::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::definitions::*;
}
