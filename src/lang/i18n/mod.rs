// devela::lang::i18n
//
//! Internationalization and localization support.
//!
//! Utilities for translating and adapting software to different languages and regions.
//

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // WIPZONE:
        // pub use super::fluent::_all::*;
        // pub use super::gettext::_all::*;
        // pub use super::msf2::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod fluent;
// mod gettext;
// mod msf2;
