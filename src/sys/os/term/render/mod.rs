// devela::sys::os::term::render
//
//! Terminal byte-frame rendering.
//

#[cfg(test)]
mod tests;

mod define; // TermRenderer

// impls
mod core_io;
mod shared;
mod exclusive;
#[cfg(feature = "alloc")]
mod owned;
mod grid;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
