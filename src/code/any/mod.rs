// devela::code::any
//
#![doc = crate::_DOC_CODE_ANY!()]
// #![doc = crate::_doc!(extends: any)]
// #![doc = crate::_doc!(modules: crate::code; any)]
// #![doc = crate::_doc!(newline)]
//

crate::mod_path!(_c "../../../libs/base/src/code/any/reexports.rs");

mod ext;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{ext::*, _c::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{ext::*, _c::*};
    }
}
