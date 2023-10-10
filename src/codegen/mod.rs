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

/* only compiled with the `codegen` feature */

#[cfg(feature = "codegen")]
mod const_for;
#[cfg(feature = "codegen")]
mod deprecate;
#[cfg(feature = "codegen")]
mod skip_format;

/* re-exports */

#[cfg(feature = "codegen")]
mod reexport;

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

#[cfg(feature = "codegen")]
pub use all::*;
#[cfg(feature = "codegen")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        const_for::const_for,
        deprecate::deprecate_feature,
        iif::iif,
        skip_format::{sf, sfb},
    };

    #[doc(inline)]
    pub use super::paste::paste;

    #[doc(inline)]
    pub use super::reexport::*;
}
