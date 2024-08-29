// devela::code
//
//! Code reflective synthesis.
#![doc = crate::code::doc_extends!(any, clone, convert, default, hint, marker, ops)]
//!
//

// safety:
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

// hidden re-exports
#[doc(hidden)]
pub use paste::__paste;

// crate private
mod _private;
pub(crate) use _private::*;

mod any; // dynamic typing and reflection
mod asserts; // assertion macros
mod cdbg; // cdbg!
mod cfor; // cfor!
mod default; // ConstDefault, Default
mod deprecate; // deprecate_feature!
mod ident; // identifier related macros
mod iif; // iif!
mod items; // items!, sf!
mod paste; // paste! wrapped for docs
mod reexports; // re-exported items
#[allow(unused_imports)]
pub use {
    any::all::*, asserts::*, cdbg::*, cfor::*, default::*, deprecate::*, ident::*, iif::*,
    items::*, paste::*, reexports::*,
};

#[cfg(_some_bit)]
items! { mod enumset; pub use enumset::*; } // enumset!

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        any::all::*, asserts::*, cdbg::*, cfor::*, default::*, deprecate::*, ident::*, iif::*,
        items::*, paste::*, reexports::*,
    };

    #[cfg(_some_bit)]
    pub use super::enumset::*;
}
