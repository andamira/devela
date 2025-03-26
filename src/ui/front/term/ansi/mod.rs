// devela::ui::front::term::ansi
//
//! ANSI codes.
//

#![allow(non_snake_case)]

mod ansi;
mod color;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{ansi::*, color::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
