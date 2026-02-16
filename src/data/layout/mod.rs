// devela::data::layout
//
#![doc = crate::_DOC_DATA_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::data; layout: array)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_QUO_DATA_LAYOUT!()]
//

mod linked; // ConstList[Item], LinkedList
mod queue;
mod stack;
// mod view;

pub mod array;
// pub mod table;

crate::structural_mods! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            linked::_all::*,
            queue::_all::*,
            stack::_all::*,
            // view::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            array::_all::*,
            // table::_all::*,
        };
    }
    _reexports {
        // buffer
        #[doc(inline)]
        pub use devela_base_core::data::layout::define_bufline;
        #[cfg(feature = "_docs_examples")]
        pub use devela_base_core::data::layout::BufLineExample;

        // sort
        #[doc(inline)]
        pub use devela_base_core::data::layout::Sort;
        // #[doc(inline)]
        // #[cfg(feature = "alloc")]
        // pub use devela_base_alloc::SortAlloc;
    }
}
