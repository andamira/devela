// devela/sys/hw/mcu/avr/timer/_.rs
//
//! AVR timers.
//

crate::mods_in! {
    mod zero;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            zero::AvrTimer0,
        };
    }
}
