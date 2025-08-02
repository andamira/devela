// devela::code::intro
//
//! Introspection.
//

mod define; // Introspect

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::define::*;
        // WIPZONE
        // pub use super::impls::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod impls;
