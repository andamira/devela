// devela::code
//
//! Code reflective synthesis.
#![doc = crate::doc_!(modules: crate; code: marker, ops, result, util)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: any, clone, convert, default, hint, marker, ops)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

mod any; // dynamic typing and reflection
mod default; // ConstDefault, Default
mod reexports; // re-exported items

pub mod marker; // core::marker, type_marker!, type_resource!, TypeResource, TypeResourced
pub mod ops; // re-exported overloadable operators
pub mod result; // AllError, serr, sok, Mismatch, OptRes, ValueQuant…
pub mod util; // utility macros and functions

crate::items! { // structural access: doc_inline, doc_hidden, items_private, all, always
    #[allow(unused)]
    pub use {doc_hidden::*, doc_inline::*, items_private::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{any::all::*, default::*, reexports::*};
    }
    mod doc_hidden {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{marker::all::*, ops::*, result::all::*, util::all::*};
    }
    pub(super) mod items_private {
        pub(crate) use super::util::items_private::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{marker::always::*, reexports::*, result::always::*, util::always::*};
    }
}
