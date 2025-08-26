// devela_base::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()]
//

mod define_error; // define_error!
mod errors; // general errors
mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{define_error::*, errors::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
