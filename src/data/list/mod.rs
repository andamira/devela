// devela::data::list
//
#![doc = crate::_DOC_DATA_LIST!()]
//!
#![doc = crate::_doc!(modules: crate::data; list: array, queue, stack)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: array, collections, vec)]
//

mod link; // ConstList[Item], LinkedList
mod of; // Oneof

pub mod array;
pub mod queue;
pub mod stack;

#[cfg(feature = "_tuple")]
pub mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut

crate::structural_mods! { // _mods, _pub_mods, _always
    _mods {
        pub use super::{link::_all::*, of::_all::*};
    }
    _pub_mods {
        pub use super::{array::_all::*, queue::_all::*, stack::_all::*};
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
    _always {
        pub use super::{array::_always::*, link::_always::*, queue::_always::*};
    }
}
