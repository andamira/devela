// devela_base_core::data::list
//
#![doc = crate::_DOC_DATA_LIST!()]
// #![doc = crate::_doc!(modules: crate::data; list: array, queue, stack)]
// #![doc = crate::_doc!(flat:"data")]
// #![doc = crate::_doc!(br+lf)]
// #![doc = crate::_doc!(extends: array)]
//

mod array;
mod buf; // define_buflist!
mod link;
mod of; // Oneof
// mod queue;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            array::_all::*,
            buf::_all::*,
            link::_all::*,
            of::_all::*,
        };
        // pub use super::queue::_all::*;
    }
}
