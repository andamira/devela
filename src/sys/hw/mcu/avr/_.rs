// devela/sys/hw/mcu/avr/_.rs
//
//! AVR microcontrollers.
//

crate::mods_in! {
    mod_ atmega328p;
    mod pin;
    mod port;
    mod register;
    // mod timer;
    mod usart;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            atmega328p::_all::Atmega328p,
            pin::AvrPin,
            port::AvrPort,
            register::AvrReg8,
            // timer::AvrTimer,
            usart::AvrUsart,
        };
    }
}
