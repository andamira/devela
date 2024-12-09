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
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        pub use super::reexports::*;
    }
}
