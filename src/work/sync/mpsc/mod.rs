// devela::work::sync::mpsc
//
//! Multi-producer, single-consumer channel.
// #![doc = crate::doc_!(extends: mpsc)] // IMPROVE

mod namespace; // Mpsc
mod reexports; // std::sync::mpsc::*

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{namespace::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
