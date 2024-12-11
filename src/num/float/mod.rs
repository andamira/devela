// devela::num::float
//
//! Floating point functionality.
//

mod alias; // fsize
mod constants; // ExtFloatConst
mod reexports;
mod wrapper; // Float

#[cfg(_float_·)]
crate::items! {
    mod ext_float; // ExtFloat
    mod shared_docs; // FORMULA_*!()
}

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{alias::*, constants::*, reexports::*, wrapper::*};

        #[cfg(_float_·)] #[allow(unused, reason = "feature-gated")]
        pub use super::{ext_float::*, wrapper::*, shared_docs::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{alias::*, reexports::*};
    }
}
