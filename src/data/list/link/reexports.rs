// devela::data::list::link::reexports

crate::mod_path!(alloc +pub _a "../../../../libs/base_alloc/src/data/list/link/reexports.rs");

pub use devela_base::ConstList;

/* ConstDefault impls */

crate::impl_cdef![<T> Self::new() => ConstList<'_, T>];

#[cfg(feature = "alloc")]
crate::impl_cdef![<T> Self::new() => LinkedList<T>];
