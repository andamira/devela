// devela::data::list::array::vec::reexports

use crate::{_reexport_from, impl_cdef};

// from workspace base
_reexport_from!(alloc
    "../../../../../libs/base_alloc/src/data/list/array/vec/reexports.rs", _a);

impl_cdef![<T> Self::new() => Vec<T>]; // impl ConstDefault
