// devela::ui::event::pointer
//
//! Pointer events.
//

mod pointer; // EventMouse, EventPointer[Type], EventButton[s|State]
mod wheel; // EventWheel,

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            pointer::*,
            wheel::*,
        };
    }
}
