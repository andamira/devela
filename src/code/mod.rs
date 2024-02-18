// devela::code
//
//! Meta-programming, extends
//! `std::{`[`any`], [`clone`]. [`convert`], [`default`], [`hint`], [`marker`], [`ops`]`}`.
//!
//! [`any`]: std::any
//! [`clone`]: std::clone
//! [`convert`]: std::convert
//! [`default`]: std::default
//! [`hint`]: std::hint
//! [`marker`]: std::marker
//! [`ops`]: std::ops
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
mod cfor; // cfor![]
mod default; // ConstDefault, Default
mod deprecate; // deprecate_feature![]
mod enumset; // enumset![]
mod ident; // identifier related macros
mod iif; // iif![]
mod paste; // paste![] wrapped for docs
mod reexports; // reexported items
mod skip_format; // sf![]

pub use {
    any::all::*, asserts::*, cfor::*, default::*, deprecate::*, enumset::*, ident::*, iif::*,
    paste::*, reexports::*, skip_format::*,
};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        any::all::*, asserts::*, cfor::*, default::*, deprecate::*, enumset::*, ident::*, iif::*,
        paste::*, reexports::*, skip_format::*,
    };
}
