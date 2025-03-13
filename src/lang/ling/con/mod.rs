// devela::lang::ling::con
//
//! Constructed languages.
//

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // WIPZONE:
        // pub use super::eo::_all::*;
        // pub use super::lojban::_all::*;
        // pub use super::toki_pona::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// pub mod eo;
// pub mod lojban;
// pub mod toki_pona;
