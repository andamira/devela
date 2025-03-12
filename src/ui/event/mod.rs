// devela::ui::event
//
//! User interface events-related functionality.
//

// mod event;
// mod window;
mod key; // EventKey, Key[Alpha|Media|Mod|Mods|Pad|State]
mod pointer; // Event[Button[State]|Mouse|Pointer[Type]|Wheel]
mod time; // EventTimestamp

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{pointer::*, key::*, time::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
