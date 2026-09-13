// devela/sys/hw/mcu/avr/timer/one/_.rs
//
//! Defines [`AvrTimer1`].
//

crate::mods_in! {
    mod define;

    // impls
    mod config; // # Configuration
    mod count; // # Counter, # Overflow
    mod compare; // # Output compare
    mod capture; // # Capture
    mod pwm; // # PWM
    mod interrupt; // # Interrupt
    mod registers; // # Semantic & Datasheet registers API
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::AvrTimer1,
        };
    }
}
