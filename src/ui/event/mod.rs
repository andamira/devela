// devela::ui::event
//
#![doc = crate::_DOC_UI_EVENT!()] // public
#![doc = crate::_doc!(modules: crate::ui; event)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

mod event; // Event
mod id; // DeviceId, WindowId
mod key; // EventKey[Ffi], Key[Ffi|Media|Mod|Mods|Pad|State]
mod kind; // EventKind, EventTag
mod pointer; // Event[Button[State]|Mouse|Pointer[Type]|Wheel]
mod queue; // EventQueue
mod target; // EventTarget
mod time; // EventTimestamp
mod window; // EventWindow

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            event::*,
            id::*,
            key::*,
            kind::*,
            pointer::*,
            queue::*,
            target::*,
            time::*,
            window::*,
        };
    }
}
