// devela::code
//
//! Code generation and meta-programming, extends
//! `std::{`[`any`], [`convert`], [`hint`], [`marker`], [`ops`]`}`.
//!
//! [`any`]: std::any
//! [`convert`]: std::convert
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
mod asserts;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod cfor;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod deprecate;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod enumset;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod ident;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod iif;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod paste;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod reexports;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "code")))]
mod skip_format;

/* re-exports */

// always compiled, crate-private
pub(crate) use _private::*;

// always compiled, hidden public
#[doc(hidden)]
pub use paste::__paste;

// always compied, non-public
pub use {
    asserts::*, cfor::*, deprecate::*, enumset::*, ident::*, iif::*, paste::*, reexports::*,
    skip_format::*,
};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        asserts::*, cfor::*, deprecate::*, enumset::*, ident::*, iif::*, paste::*, reexports::*,
        skip_format::*,
    };
}
