// devela_base::data::list
//
#![doc = crate::_DOC_DATA_LIST!()]
//

mod array;
mod link;
// mod queue;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{array::_all::*, link::_all::*};
        // pub use super::queue::_all::*;
    }
}
