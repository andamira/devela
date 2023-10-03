// devela::codegen
//
//! Code generation and meta-programming.
//

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{cif, compile, compile_attr, const_for, iif, paste};
}

pub use ::devela_macros::{cif, compile, compile_attr};

mod const_for;
mod deprecate;
mod iif;
mod paste;
mod skip_format;

pub use const_for::const_for;
pub use deprecate::deprecate_feature;
pub use iif::iif;
pub use paste::paste;
pub use skip_format::{sf, sfb};

#[doc(hidden)]
pub use paste::__paste;
