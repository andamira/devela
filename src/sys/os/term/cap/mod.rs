// devela::sys::os::term::cap
//
//! Terminal capabilities.
//

mod cap; // TermCap, TermCaps
mod color; // TermColorDepth

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cap::*,
            color::*,
        };
    }
}
