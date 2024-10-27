// devela::code
//
//! Code reflective synthesis.
#![doc = crate::code::doc_!(extends: any, clone, convert, default, hint, marker, ops)]
#![doc = crate::code::doc_!(modules: crate; code)]
#![doc = crate::code::doc_!(newline)]
//!
//

// safety:
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

// crate private
mod _private;
pub(crate) use _private::*;

mod any; // dynamic typing and reflection
mod default; // ConstDefault, Default
mod macros; // macros: assert*, cdbg, head, items, paste, sf…
mod reexports; // re-exported items
mod r#type; // type_marker!, type_resource, TypeResource, TypeResourced
#[allow(unused_imports)]
pub use {any::all::*, default::*, macros::*, r#type::*, reexports::*};

#[cfg(_some_bit)]
items! { mod enumset; pub use enumset::*; } // enumset!

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{any::all::*, default::*, macros::*, r#type::*, reexports::*};

    #[cfg(_some_bit)]
    pub use super::enumset::*;
}
