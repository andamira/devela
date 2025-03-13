// devela::lang::ling::nat
//
//! Natural languages.
//

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // WIPZONE:
        // pub use super::en::_all::*;
        // pub use super::es::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// pub mod en;
// pub mod es;
