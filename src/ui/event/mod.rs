// devela::ui::event
//
//! User interface events-related functionality.
//

mod event; // Event, EventKind
mod key; // EventKey, Key[Alpha|Media|Mod|Mods|Pad|State]
mod pointer; // Event[Button[State]|Mouse|Pointer[Type]|Wheel]
mod time; // EventTimestamp
mod window; // EventWindow

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            event::*,
            pointer::*,
            key::*,
            time::*,
            window::*,
        };
    }
}
