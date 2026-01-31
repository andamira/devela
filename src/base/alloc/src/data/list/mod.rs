// devela_base_alloc::data::list
//
#![doc = crate::_DOC_DATA_LIST!()] // public
#![doc = crate::_doc!(modules: crate::data; list: array)] // queue, stack
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, vec)]
//

mod link;

pub mod array;
// pub mod queue;
// pub mod stack;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            link::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            array::_all::*,
            // queue::_all::*,
            // stack::_all::*,
        };
    }
}
