// devela::code
//
//! Code reflective synthesis, <small>extends
//! `std::{`[`any`], [`clone`], [`convert`], [`default`], [`error`], [`hint`],
//! [`marker`], [`ops`], [`option`], [`panic`], [`result`]`}`.</small>
//!
//! [`any`]: std::any
//! [`clone`]: std::clone
//! [`convert`]: std::convert
//! [`default`]: std::default
//! [`error`]: std::error
//! [`hint`]: std::hint
//! [`marker`]: std::marker
//! [`ops`]: std::ops
//! [`option`]: std::option
//! [`panic`]: mod@std::panic
//! [`result`]: std::result
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
mod cdbg; // cdbg![]
mod cfor; // cfor![]
mod default; // ConstDefault, Default
mod deprecate; // deprecate_feature![]
mod ident; // identifier related macros
mod iif; // iif![]
mod paste; // paste![] wrapped for docs
mod reexports; // re-exported items
mod result; // std::{error, option, panic, result}
mod skip_format; // sf![]
#[allow(unused_imports)]
pub use {
    any::all::*, asserts::*, cdbg::*, cfor::*, default::*, deprecate::*, ident::*, iif::*,
    paste::*, reexports::*, result::all::*, skip_format::*,
};

#[cfg(feature = "_-bit_any-_")]
mod enumset; // enumset![]
#[cfg(feature = "_-bit_any-_")]
pub use enumset::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        any::all::*, asserts::*, cdbg::*, cfor::*, default::*, deprecate::*, ident::*, iif::*,
        paste::*, reexports::*, result::all::*, skip_format::*,
    };

    #[cfg(feature = "_-bit_any-_")]
    pub use super::enumset::*;
}
