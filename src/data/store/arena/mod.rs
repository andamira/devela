// devela/src/data/store/arena/mod.rs
//
//!
//

mod byte; // arena!

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            byte::_all::*,
        };
    }
    _hidden {
        pub use super::{
            byte::_hidden::*,
        };
    }
}
