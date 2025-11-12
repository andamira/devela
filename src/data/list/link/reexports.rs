// devela::data::list::link::reexports

crate::mod_path!(alloc +pub _a "../../../../libs/base_alloc/src/data/list/link/reexports.rs");

#[doc(inline)]
pub use devela_base_core::ConstList;

/* ConstInit impls */

crate::_impl_init![ConstInit: <T> Self::new() => ConstList<'_, T>];

#[cfg(feature = "alloc")]
crate::_impl_init![ConstInit: <T> Self::new() => LinkedList<T>];
