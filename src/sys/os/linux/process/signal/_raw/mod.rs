// devela/src/sys/os/linux/process/signal/_raw/mod.rs

// #![allow(
//     dead_code,
//     non_camel_case_types,
//     clippy::upper_case_acronyms,
//     clippy::zero_prefixed_literal
// )]
#![allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]

mod action; // LINUX_SIGACTION
mod signal; // LINUX_SIGNAL

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{
            action::*,
            signal::*,
        };
    }
}
