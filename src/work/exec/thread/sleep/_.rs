// devela/src/work/exec/thread/sleep/_.rs
//
//! Thread sleeping functionality.
//

crate::mods_in! {
    mod r#macro; // sleep4!
    // pub use sleeper::*; // TODO
    // pub use spin::*; // TODO
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            r#macro::*, // NOTE: keep the wildcard
            // sleeper::Sleeper,
            // spin::Spin,
        };
    }
}
