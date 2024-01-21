// devela::code
//
//! Code generation and meta-programming, extends
//! `std::{`[`convert`], [`hint`], [`marker`]`}`.
//!
//! [`convert`]: core::convert
//! [`hint`]: core::hint
//! [`marker`]: core::marker
//

/* contains always compiled items */

// crate internal use only
mod _private;
#[allow(unused)]
pub(crate) use _private::reexport;

// internal and external use
mod chain;
mod enumset;
mod r#for;
mod ident;
mod iif;
mod paste;
mod reexports;
mod skip_format;
#[allow(unused)]
#[cfg(not(feature = "code"))]
pub use {
    chain::*, enumset::*, ident::*, iif::*, paste::*, r#for::*, reexports::*, skip_format::*,
};

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

/* feature-gated */

#[cfg(feature = "code")]
mod deprecate;

// re-export private sub-modules
#[cfg(feature = "code")]
pub use {
    chain::*, deprecate::*, enumset::*, ident::*, iif::*, paste::*, r#for::*, reexports::*,
    skip_format::*,
};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        chain::*, enumset::*, ident::*, iif::*, paste::*, r#for::*, reexports::*, skip_format::*,
    };

    #[doc(inline)]
    #[cfg(feature = "code")]
    pub use super::deprecate::*;
}
