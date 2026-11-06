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
        #[cfg(feature = "std")]
        pub use super::r#macro::sleep4;
        #[cfg(all(not(feature = "std"), feature = "time"))]
        #[crate::macro_apply(crate::_linux_syscall)]
        pub use super::r#macro::sleep4;

        // pub use super::{
        //     // sleeper::Sleeper,
        //     // spin::Spin,
        // };
    }
}
