//
//! AVR ADC access and weak physical-noise harvesting.
//

crate::mods_in! {
    mod define;
    mod input;
    mod noise;

    // mod conversion; // conversion operations
    mod registers; // semantic + datasheet register access
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::AvrAdc,
            input::AvrAdcInput,
            noise::AvrAdcNoise,
        };
    }
}
