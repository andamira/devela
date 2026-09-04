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
        #[crate::macro_apply(crate::_std_or_linux_syscall)]
        pub use super::r#macro::sleep4;

        // pub use super::{
        //     // sleeper::Sleeper,
        //     // spin::Spin,
        // };
    }
}
