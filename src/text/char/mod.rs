// devela::text::char
//
//! Unicode scalars.
// #![doc = crate::doc_!(extends: char)]
// #![doc = crate::doc_!(modules: crate::text; char)]
// #![doc = crate::doc_!(newline)]
//

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

// with re-exports
mod definitions;
mod namespace;
mod reexports;
#[allow(unused_imports)]
pub use {definitions::*, namespace::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{definitions::*, namespace::*, reexports::*};
}
