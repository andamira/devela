// devela::codegen
//
//! Code generation and meta-programming.
//

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{cif, compile, compile_attr, paste};
}

pub use ::devela_macros::{cif, compile, compile_attr};

mod deprecate;
mod iif;
mod paste;

pub use deprecate::deprecate_feature;
pub use iif::iif;
pub use paste::paste;

#[doc(hidden)]
pub use paste::__paste;
