// devela::data::list::queue::reexports

crate::mod_path!(alloc +pub _a "../../../../libs/base_alloc/src/data/list/queue/reexports.rs");

#[cfg(feature = "alloc")]
crate::impl_cdef![<T> Self::new() => VecDeque<T>]; // impl ConstDefault
