// devela::codegen
//
//! Code generation and meta-programming.
//

/* always compiled for internal use */

mod iif;
mod paste;

#[allow(unused)]
#[cfg(not(feature = "codegen"))]
pub(crate) use {iif::iif, paste::paste};

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

/* only compiled with the `codegen` feature */

#[cfg(feature = "codegen")]
mod const_for;
#[cfg(feature = "codegen")]
mod deprecate;
#[cfg(feature = "codegen")]
mod skip_format;

/* re-exports */

#[cfg(feature = "codegen")]
pub use all::*;
#[cfg(feature = "codegen")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        const_for::const_for,
        deprecate::deprecate_feature,
        iif::iif,
        paste::paste,
        skip_format::{sf, sfb},
    };

    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(feature = "codegen", feature = "devela_macros")))
    )]
    pub use ::devela_macros::{cif, compile, compile_attr};
}
