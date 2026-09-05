// devela/src/sys/os/linux/process/signal/_raw/_.rs
//
//!
//

// #![allow(
//     dead_code,
//     non_camel_case_types,
//     clippy::upper_case_acronyms,
//     clippy::zero_prefixed_literal
// )]
#![allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]

crate::mods_in! {
    mod action; // LINUX_SIGACTION
    mod signal; // LINUX_SIGNAL
}
crate::mods_out! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{
            action::*,
            signal::*,
        };
    }
}
