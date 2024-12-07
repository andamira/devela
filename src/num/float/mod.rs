// devela::num::float
//
//! Floating point functionality.
//

mod alias; // fsize
mod constants; // ExtFloatConst
mod reexports;

#[cfg(_float_·)]
crate::items! {
    mod ext_float; // ExtFloat
    mod wrapper; // Float
    mod shared_docs; // FORMULA_*!()
}

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{alias::*, constants::*, reexports::*};

        #[cfg(_float_·)] #[allow(unused_imports)]
        pub use super::{ext_float::*, wrapper::*, shared_docs::*};
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
