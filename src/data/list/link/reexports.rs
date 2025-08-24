// devela::data::list::link::reexports

use crate::_reexport_from;

// from workspace base
_reexport_from!(alloc "../../../../libs/base_alloc/src/data/list/link/reexports.rs", _c);

#[cfg(feature = "alloc")]
crate::impl_cdef![<T> Self::new() => LinkedList<T>]; // impl ConstDefault
