// devela/src/ui/event/pointer/_.rs
//
//! Pointer events.
//

crate::mods_in! {
    mod button; // EventButton[s|State]
    mod pointer; // EventMouse, EventPointer[Type]
    mod wheel; // EventWheel[Unit]
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            button::*,
            pointer::*,
            wheel::*,
        };
    }
}
