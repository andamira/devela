// devela::sys::os::linux::structs
//
//! Linux related structs.
//

mod sigaction;
mod stat;
mod timespec;
#[allow(unused)]
pub use {sigaction::*, stat::*, timespec::*};

#[cfg(feature = "term")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
    mod termios;
    pub use termios::*;
}
