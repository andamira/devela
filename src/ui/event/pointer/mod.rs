// devela/src/ui/event/pointer/mod.rs
//
//! Pointer events.
//

mod button; // EventButton[s|State]
mod pointer; // EventMouse, EventPointer[Type]
mod wheel; // EventWheel[Unit]

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            button::*,
            pointer::*,
            wheel::*,
        };
    }
}
