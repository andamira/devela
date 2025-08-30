// devela::ui::event
//
//! User interface events-related functionality.
//

// mod event;
// mod window;
mod key; // EventKey, Key[Alpha|Media|Mod|Mods|Pad|State]
mod pointer; // Event[Button[State]|Mouse|Pointer[Type]|Wheel]
mod time; // EventTimestamp

crate::structural_mods! { // _mods
    _mods {
        pub use super::{pointer::*, key::*, time::*};
    }
}
