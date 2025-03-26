// devela::ui::front::term
//
//! Terminal functionality.
//

mod size;
mod ansi;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::size::*;
        pub use super::ansi::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
