// devela/src/work/exec/thread/sleep/mod.rs
//
//! Thread sleeping functionality.
//

mod r#macro; // sleep4!
// pub use sleeper::*; // WIP
// pub use spin::*; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::r#macro::*;
        // mod sleeper; // Sleeper
        // mod spin; // Spin
    }
}
