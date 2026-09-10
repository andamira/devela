// devela/sys/hw/mcu/avr/atmega328p/_.rs
//
//! ATmega328P microcontroller.
//

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::Atmega328p,
        };
    }
}
