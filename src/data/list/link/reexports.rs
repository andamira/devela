// devela::data::list::link::reexports

use crate::{_reexport_from, impl_cdef};

// from workspace base
_reexport_from!(alloc "../../../../libs/base_alloc/src/data/list/link/reexports.rs", _c);

#[cfg(feature = "alloc")]
impl_cdef![<T> Self::new() => LinkedList<T>]; // impl ConstDefault
