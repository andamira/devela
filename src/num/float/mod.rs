// devela::num::float
//
//! Floating point functionality.
//

mod constants; // ExtFloatConst
mod reexports;

#[cfg(_float_·)]
crate::items! {
    mod ext_float; // ExtFloat
    mod wrapper; // Float
    mod shared_docs; // FORMULA_*!()
}

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{constants::*, reexports::*};
        #[cfg(_float_·)] #[allow(unused_imports)]
        pub use super::{ext_float::*, wrapper::*, shared_docs::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
