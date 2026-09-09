// devela/src/code/util/debug/_.rs
//
#![doc = crate::_DOC_CODE_UTIL_DEBUG!()] // public
#![doc = crate::_doc!(modules: crate::code::util; debug)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: backtrace)]
//

crate::mods_in! {
    #[cfg(feature = "std")]
    mod _reexport_std;

    mod cdbg;
    mod fn_name;
    mod warn;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use super::{
            cdbg::cdbg,
            fn_name::fn_name,
            warn::const_warn,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
