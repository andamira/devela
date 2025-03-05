// devela::work::future::coroutine
//
//! Coroutine implementations.
//

mod coro;

#[cfg(nightly_coro)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(nightly_coro)))]
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::coro::*;

        #[cfg(nightly_coro)]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(nightly_coro)))]
        pub use super::reexports::*;
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(nightly_coro)]
        pub use super::reexports::*;
    }
}
