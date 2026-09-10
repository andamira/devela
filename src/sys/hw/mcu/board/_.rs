// devela/sys/hw/mcu/board/_.rs
//
//! Microcontroller boards.
//

crate::mods_in! {
    mod_ arduino;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            arduino::_all::ArduinoNano,
        };
    }
}
