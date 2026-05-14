// devela::sys::os::term::event

mod input;
// mod signal;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            input::*,
            // signal::*,
        };
    }
}
