// devela::work::thread::sleep
//
//! Thread sleeping functionality.
//

mod macros; // sleep4!

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::macros::*;
        // WIPZONE
        // mod sleeper; // Sleeper
        // mod spin; // Spin
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// pub use sleeper::*;
// pub use spin::*;
