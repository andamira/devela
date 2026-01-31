// devela::data::list
//
#![doc = crate::_DOC_DATA_LIST!()] // public
#![doc = crate::_doc!(modules: crate::data; list: array, queue, stack)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, vec)]
//

mod link; // ConstList[Item], LinkedList
mod of; // Oneof

pub mod array;
pub mod queue;
pub mod stack;

#[cfg(feature = "_tuple")]
pub mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut

crate::structural_mods! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            link::_all::*,
            of::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            array::_all::*,
            queue::_all::*,
            stack::_all::*,
        };
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::data::list::define_bufline;
        #[cfg(feature = "_docs_min")]
        pub use devela_base_core::data::list::BufLineExample;
    }
}
