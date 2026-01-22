// devela_base_alloc::data::list
//
#![doc = crate::_DOC_DATA_LIST!()]
//

mod array;
// mod queue;

crate::structural_mods! { // _mods
    _mods {
        pub use super::array::_all::*;
        // pub use super::queue::_all::*;
    }
}
