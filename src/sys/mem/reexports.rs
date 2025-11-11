// devela::sys::mem::reexports
//
//!
//

crate::mod_path!(+pub _c "../../../libs/base_core/src/sys/mem/reexports.rs");
crate::mod_path!(alloc +pub _a "../../../libs/base_alloc/src/sys/mem/reexports.rs");

#[doc(inline)]
pub use devela_base_core::{CacheAlign, MaybeByte, Mem, MemAligned, cswap, define_arena};

#[cfg(feature = "alloc")]
crate::impl_cdef![ConstDefault: <T: ConstDefault> Self::new() => RcWeak<T>];

#[doc(inline)]
pub use crate::Sized;
