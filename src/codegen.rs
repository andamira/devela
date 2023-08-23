// devela::codegen
//
//! Code generation and meta-programming.
//

/// Reexported from the [`paste`](https://docs.rs/paste) crate.
/// Allows to paste identifiers in a macro.
pub use paste::paste;

pub use ::devela_macros::{cif, compile, compile_attr};
