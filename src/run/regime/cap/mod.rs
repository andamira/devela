// devela/src/run/regime/cap/mod.rs
//
//! Runtime capabilities.
//

mod define; // RunCap[Audio|Color|Image|Input|System|Window]

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::*;
    }
}
