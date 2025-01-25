// devela::data::table
//
//! Tabular and heterogeneous data processing.
//

pub mod value;

crate::items! { // structural access: _pub_mods, _all, _internal
    #[allow(unused)]
    pub use _internals::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        pub use super::value::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::value::_internals::*;
    }
}
