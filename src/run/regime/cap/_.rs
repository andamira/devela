// devela/src/run/regime/cap/_.rs
//
//! Runtime capabilities.
//

crate::mods_in! {
    mod define; // RunCap[Audio|Color|Image|Input|System|Window]
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::*;
    }
}
