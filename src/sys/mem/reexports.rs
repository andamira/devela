// devela::sys::mem::reexports
//
//!
//

// from workspace base
crate::_reexport_from!("../../../libs/base/src/sys/mem/reexports.rs", _c);
crate::_reexport_from!(alloc "../../../libs/base_alloc/src/sys/mem/reexports.rs", _a);

#[cfg(feature = "alloc")]
crate::impl_cdef![<T: ConstDefault> Self::new() => RcWeak<T>];

#[doc(inline)]
pub use crate::Sized;
