// devela::num::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(prim··)]
crate::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(prim··)))]
    mod namespace; // Cast

    #[cfg(test)]
    mod tests;
}

#[cfg(feature = "cast")]
mod cast; // PrimitiveCast
#[cfg(feature = "join")]
mod join; // PrimitiveJoin
#[cfg(feature = "split")]
mod split; // PrimitiveSplit

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        #[cfg(prim··)]
        pub use super::namespace::*;

        #[cfg(feature = "cast")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "cast")))]
        pub use super::cast::*;

        #[cfg(feature = "join")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "join")))]
        pub use super::join::*;

        #[cfg(feature = "split")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "split")))]
        pub use super::split::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
