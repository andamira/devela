// devela::code
//
//! Code generation and meta-programming, extends
//! `std::{`[`convert`], [`hint`], [`marker`]`}`.
//!
//! [`convert`]: core::convert
//! [`hint`]: core::hint
//! [`marker`]: core::marker
//

#![cfg_attr(not(feature = "code"), allow(unused_imports))]

/* modules */

// always compiled, crate-private
mod _private;

// always compiled, non-public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod chain;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod deprecate;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod enumset;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod r#for;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod ident;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod iif;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod paste;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod reexports;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
mod skip_format;

/* re-exports */

// always compiled, crate-private
pub(crate) use _private::*;

// always compiled, hidden public
#[doc(hidden)]
pub use paste::__paste;

// always compied, non-public
pub use {
    chain::*, deprecate::*, enumset::*, ident::*, iif::*, paste::*, r#for::*, reexports::*,
    skip_format::*,
};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        chain::*, deprecate::*, enumset::*, ident::*, iif::*, paste::*, r#for::*, reexports::*,
        skip_format::*,
    };
}
