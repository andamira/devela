// devela_macros::bodies
//
//! The bodies of the proc_macro functions defined in `lib.rs`.
//

mod shared;

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests;

mod compile;
mod ident;
mod niche;
pub(crate) use {compile::*, ident::*, niche::*};
