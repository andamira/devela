// devela::data::list::link::reexports

crate::_reexport_from!(alloc "../../../../libs/base_alloc/src/data/list/link/reexports.rs", _c);

pub use devela_base::ConstList;

/* ConstDefault impls */

crate::impl_cdef![<T> Self::new() => ConstList<'_, T>];

#[cfg(feature = "alloc")]
crate::impl_cdef![<T> Self::new() => LinkedList<T>];
