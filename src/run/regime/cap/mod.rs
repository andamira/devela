// devela::run::regime::cap
//
//! Runtime capabilities.
//

mod definitions; // RunCap[Audio|Image|Input|System|Window]

crate::structural_mods! { // _mods
    _mods {
        pub use super::definitions::*;
    }
}
