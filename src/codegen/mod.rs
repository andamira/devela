// devela::codegen
//
//! Code generation and meta-programming.
//

/* always compiled for internal use */

// just private:
mod _private;
#[allow(unused)]
pub(crate) use _private::reexport;

// both private and public:
mod iif;
mod paste;
mod skip_format;
#[allow(unused)]
#[cfg(not(feature = "codegen"))]
pub(crate) use {iif::iif, paste::paste, skip_format::sf};

#[doc(hidden)]
#[allow(unused)]
pub use paste::__paste;

/* only compiled with the `codegen` feature */

#[cfg(feature = "codegen")]
mod const_for;
#[cfg(feature = "codegen")]
mod deprecate;

/* re-exports */

#[cfg(feature = "codegen")]
mod reexport;

#[cfg(feature = "codegen")]
pub use all::*;
#[cfg(feature = "codegen")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        const_for::const_for, deprecate::deprecate_feature, iif::iif, skip_format::sf,
    };

    #[doc(inline)]
    pub use super::paste::paste;

    #[doc(inline)]
    pub use super::reexport::*;
}
