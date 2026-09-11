// devela/sys/hw/mcu/avr/timer/_.rs
//
//! Avr timers.
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
