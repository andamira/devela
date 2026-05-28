// devela::sys::os::term::event

mod input;
// mod signal;

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            input::_all::*,
            // signal::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            input::_crate_internals::*,
        };
    }
}
