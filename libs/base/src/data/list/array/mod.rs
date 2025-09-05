// devela_base::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//

mod from; // ArrayFrom
mod init; // array_init!
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            from::*,
            init::*,
            reexports::*,
        };
    }
}
