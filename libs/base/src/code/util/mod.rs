// devela_base::code::util
//
//! Utility macros and hint functions.
//

mod items; // items!, sf!

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{
            items::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
