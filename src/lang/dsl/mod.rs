// devela::lang::dsl
//
//! Custom <abbr title = "Domain Specific Language">DSL</abbr>s.
//

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // WIPZONE:
        // pub use super::apl::_all::*;
        // pub use super::awk::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// pub mod apl;
// pub mod awk;
