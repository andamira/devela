// devela/src/code/util/debug/mod.rs
//
#![doc = crate::_DOC_CODE_UTIL_DEBUG!()] // public
#![doc = crate::_doc!(modules: crate::code::util; debug)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//

mod cdbg;
mod fn_name;
mod warn;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            cdbg::cdbg,
            fn_name::fn_name,
            warn::const_warn,
        };
    }
}
