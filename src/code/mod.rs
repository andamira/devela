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

// warnings:
#![cfg_attr(not(feature = "code"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

/* hidden re-exports */

#[doc(hidden)]
pub use paste::__paste;

/* always compiled, crate-private modules */

mod _private;

pub(crate) use _private::*;

/* always compiled, non-public modules */

mod any; // dynamic typing and reflection
mod asserts; // assertion macros
mod cdbg; // cdbg![]
mod cfor; // cfor![]
mod default; // ConstDefault, Default
mod deprecate; // deprecate_feature![]
mod enumset; // enumset![]
mod ident; // identifier related macros
mod iif; // iif![]
mod paste; // paste![] wrapped for docs
mod reexports; // re-exported items
mod result; // std::{error, option, panic, result}
mod skip_format; // sf![]

pub use {
    any::all::*, asserts::*, cdbg::*, cfor::*, default::*, deprecate::*, enumset::*, ident::*,
    iif::*, paste::*, reexports::*, result::all::*, skip_format::*,
};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        any::all::*, asserts::*, cdbg::*, cfor::*, default::*, deprecate::*, enumset::*, ident::*,
        iif::*, paste::*, reexports::*, result::all::*, skip_format::*,
    };
}
