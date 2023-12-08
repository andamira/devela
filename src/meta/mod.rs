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
mod const_for;
mod iif;
mod paste;
mod skip_format;
mod reexports;

#[allow(unused)]
#[cfg(not(feature = "meta"))]
pub(crate) use {const_for::cfor, iif::iif, paste::paste, reexports::*, skip_format::sf};

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

/* feature-gated */

#[cfg(feature = "meta")]
mod deprecate;

// re-export private sub-modules
#[cfg(feature = "meta")]
pub use {
    const_for::cfor, deprecate::deprecate_feature, iif::iif, paste::paste, reexports::*,
    skip_format::sf,
};

#[cfg(feature = "meta")]
pub(crate) mod all {
    pub use super::{
        const_for::cfor, deprecate::deprecate_feature, iif::iif, paste::paste, reexports::*,
        skip_format::sf,
    };
}
