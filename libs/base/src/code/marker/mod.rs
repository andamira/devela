// devela_base::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()]
//

mod reexports;
mod type_marker; // zero-cost generic type markers

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{type_marker::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
