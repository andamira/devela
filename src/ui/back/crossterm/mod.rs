// devela::ui::back::crossterm
//
//! [`crossterm`][crate::_dep::crossterm] UI backends.
//

mod service;
// mod events;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::service::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
