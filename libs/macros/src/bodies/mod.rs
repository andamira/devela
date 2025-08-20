// devela_macros::bodies
//
//! The bodies of the proc_macro functions defined in `lib.rs`.
//

mod shared;
mod enumint; // enumint!

pub(crate) use enumint::*;
