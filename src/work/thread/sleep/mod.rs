// devela::work::thread::sleep
//
//! Thread sleeping functionality.
//

mod macros; // sleep4!

// WIPZONE
// pub use sleeper::*;
// pub use spin::*;

crate::structural_mods! { // _mods
    _mods {
        pub use super::macros::*;
        // WIPZONE
        // mod sleeper; // Sleeper
        // mod spin; // Spin
    }
}
