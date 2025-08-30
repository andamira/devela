// devela_base::work::future
//
#![doc = crate::_DOC_WORK_FUTURE!()]
//

mod coroutine;
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            coroutine::_all::*,
            reexports::*,
        };
    }
}
