// devela::meta
//
//! Meta-programming and code generation.
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
#[cfg(not(feature = "meta"))]
pub(crate) use {iif::iif, paste::paste, skip_format::sf};

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

/* feature-gated */

#[cfg(feature = "meta")]
mod const_for;
#[cfg(feature = "meta")]
mod deprecate;
#[cfg(feature = "meta")]
mod reexports;

// re-export private sub-modules
#[cfg(feature = "meta")]
pub use {
    const_for::const_for, deprecate::deprecate_feature, iif::iif, paste::paste, reexports::*,
    skip_format::sf,
};

#[cfg(feature = "meta")]
pub(crate) mod all {
    pub use super::{
        const_for::const_for, deprecate::deprecate_feature, iif::iif, paste::paste, reexports::*,
        skip_format::sf,
    };
}
