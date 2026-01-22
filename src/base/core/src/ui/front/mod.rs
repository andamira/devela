// devela_base_core::ui::front
//
//! UI frontends.
//

pub mod term;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::term::_all::*;
    }
    _crate_internals {
        pub use super::term::_crate_internals::*;
    }
}
