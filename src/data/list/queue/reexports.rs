// devela::data::list::queue::reexports

use crate::_reexport_from;

// from workspace base
_reexport_from!(alloc "../../../../libs/base_alloc/src/data/list/queue/reexports.rs", _c);

#[cfg(feature = "alloc")]
crate::impl_cdef![<T> Self::new() => VecDeque<T>]; // impl ConstDefault
