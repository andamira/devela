// devela::text::fmt
//
//! Formatting strings.
//!
#![doc = crate::doc_!(extends: fmt)]
//

mod reexports;
mod buf;

#[cfg(feature = "fmt")]
mod num_to_str;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        pub use super::{buf::*, reexports::*};
        #[cfg(feature = "fmt")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "fmt")))]
        pub use super::num_to_str::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
