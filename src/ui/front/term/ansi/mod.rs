// devela::ui::front::term::ansi
//
//! ANSI codes.
//

#![allow(non_snake_case)]

mod color; // AnsiColor3b, AnsiColor8b
mod namespace; // Ansi
mod r#macro; // ansi!

mod print;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{color::*, namespace::*, r#macro::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
