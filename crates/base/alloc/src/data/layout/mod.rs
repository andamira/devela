// devela_base_alloc::data::layout
//
#![doc = crate::_DOC_DATA_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::data; layout: array)] // space
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, vec)]
#![doc = crate::_QUO_DATA_LAYOUT!()]
//

mod buffer;
// mod pool;
// mod queue;
mod sort; // Sort
// mod stack;

pub mod array;
// pub mod space;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            buffer::_all::*,
            // pool::_all::*,
            // queue::_all::*,
            sort::_all::*,
            // stack::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            array::_all::*,
            // space::_all::*,
        };
    }
}
