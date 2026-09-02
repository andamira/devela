// devela/src/sys/os/term/event/mod.rs

mod input;

crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            input::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            input::_crate_internals::*,
        };
    }
}
