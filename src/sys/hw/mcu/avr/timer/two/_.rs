// devela/sys/hw/mcu/avr/timer/two/_.rs
//
//! Defines [`AvrTimer2`].
//

crate::mods_in! {
    mod define;

    // impls
    mod config; // # Configuration
    mod count; // # Counter, # Overflow
    mod compare; // # Output compare
    mod interrupt; // # Interrupt
    // mod pwm; // # PWM
    mod registers; // # Semantic & Datasheet registers API
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::AvrTimer2,
        };
    }
}
