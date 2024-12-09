// devela::code
//
//! Code reflective synthesis.
#![doc = crate::doc_!(modules: crate; code: macros, ops, result)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: any, clone, convert, default, hint, marker, ops)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

mod any; // dynamic typing and reflection
mod default; // ConstDefault, Default
mod r#type; // type_marker!, type_resource, TypeResource, TypeResourced
mod reexports; // re-exported items

pub mod macros; // macros: assert*, cdbg, head, items, paste, sf…
pub mod ops; // re-exported overloadable operators
pub mod result;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {doc_hidden::*, doc_inline::*, items_private::*};

    mod doc_inline {
        pub use super::{any::all::*, default::*, r#type::*, reexports::*};
    }
    mod doc_hidden { #[doc(hidden)] #[doc(no_inline)]
        pub use super::{macros::all::*, ops::*, result::all::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused_imports)]
        pub use super::{macros::always::*, reexports::*};
    }
    pub(super) mod items_private {
        pub(crate) use super::macros::items_private::*;
    }
}
