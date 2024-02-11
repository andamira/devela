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

/* modules */

// always compiled, crate-private
mod _private;

// always compiled, non-public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod any; // dynamic typing and reflection
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod asserts; // assertion macros
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod cfor; // cfor![]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod default; // ConstDefault, Default
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod deprecate; // deprecate_feature![]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod enumset; // enumset![]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod ident; // identifier related macros
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod iif; // iif![]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod paste; // paste![] wrapped for docs
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod reexports; // reexported items
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod skip_format; // sf![]

/* re-exports */

// always compiled, crate-private
pub(crate) use _private::*;

// always compiled, hidden public
#[doc(hidden)]
pub use paste::__paste;

// always compied, non-public
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
