// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

#[cfg(test)]
mod tests;

mod non_value;
mod reexports;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{non_value::*, reexports::*};
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
