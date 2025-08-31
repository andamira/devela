// devela::ui::back::crossterm
//
//! [`crossterm`][crate::_dep::crossterm] UI backends.
//

mod service;

// WIPZONE
// mod events;

crate::structural_mods! { // _mods
    _mods {
        pub use super::service::*;

        // WIPZONE
        // pub use super::events::*;
    }
}
