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

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_pub_mods::*, _always::*};

    mod _mods {
        pub use super::{link::_all::*, of::_all::*};
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{array::_all::*, queue::_all::*, stack::_all::*};
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{array::_always::*, link::_always::*, queue::_always::*};
    }
}
