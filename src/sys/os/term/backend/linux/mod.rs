// devela::sys::os::term::backend::linux
//
//! Linux terminal backend.
//

mod term; // TermLinux
mod restore; // (TermLinuxRestore, TermLinuxRestoreFlags)

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::term::*;
    }
    _crate_internals {
        pub use super::restore::*;
    }
}
