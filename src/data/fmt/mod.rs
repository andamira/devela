// devela::data::serde
//
//! Data serialization and deserialization.
//

mod reexports;
mod types;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{reexports::*, types::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_mods::*;
    }
}
