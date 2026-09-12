// devela/sys/hw/mcu/avr/timer/_.rs
//
//! AVR timers.
//

crate::mods_in! {
    mod one;
    mod zero;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            one::AvrTimer1,
            zero::AvrTimer0,
        };
    }
}
