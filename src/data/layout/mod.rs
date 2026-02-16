// devela::data::layout
//
#![doc = crate::_DOC_DATA_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::data; layout: array, dst)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_QUO_DATA_LAYOUT!()]
//

mod collection; // DataCollection
mod linked; // ConstList[Item], LinkedList
mod queue;
mod stack;
// mod view;

pub mod array;
// pub mod table;

#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst;

crate::structural_mods! { // _mods, _pub_mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            collection::*,
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
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub use super::dst::_all::*;
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
    _crate_internals {
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub(crate) use super::dst::_crate_internals::*;
    }
}
