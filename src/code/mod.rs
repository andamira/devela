// devela::code
//
//! Code reflective synthesis.
#![doc = crate::doc_!(modules: crate; code: macros, result)]
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
pub use {any::all::*, default::*, r#type::*, reexports::*};

pub mod macros; // macros: assert*, cdbg, head, items, paste, sf…
pub mod result;
#[doc(no_inline)]
#[allow(unused_imports)]
pub use {macros::*, result::all::*};

pub(super) mod all {
    #![allow(unused_imports)]
    #[doc(inline)]
    pub use super::{any::all::*, default::*, macros::*, r#type::*, reexports::*, result::all::*};
}
