// devela_base::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()]
//

mod mismatch; // Mismatch
mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{mismatch::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
