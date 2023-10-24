// devela::codegen
//
//! Code generation and meta-programming.
//

/* contains always compiled items */

// internal use only
mod _private;

#[allow(unused)]
pub(crate) use _private::reexport;

// internal and external use
mod iif;
mod paste;
mod skip_format;

#[allow(unused)]
#[cfg(not(feature = "codegen"))]
pub(crate) use {iif::iif, paste::paste, skip_format::sf};

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

/* feature-gated */

#[cfg(feature = "codegen")]
mod const_for;
#[cfg(feature = "codegen")]
mod deprecate;
#[cfg(feature = "codegen")]
mod reexports;

// re-export private sub-modules
#[cfg(feature = "codegen")]
pub use {
    const_for::const_for, deprecate::deprecate_feature, iif::iif, paste::paste, reexports::*,
    skip_format::sf,
};

#[cfg(feature = "codegen")]
pub(crate) mod all {
    pub use super::{
        const_for::const_for, deprecate::deprecate_feature, iif::iif, paste::paste, reexports::*,
        skip_format::sf,
    };
}
