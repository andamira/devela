// devela::sys::mem::reexports
//
//!
//

crate::mod_path!(+pub _c "../../../libs/base_core/src/sys/mem/reexports.rs");
crate::mod_path!(alloc +pub _a "../../../libs/base_alloc/src/sys/mem/reexports.rs");

pub use devela_base_core::{Mem, cswap};

#[cfg(feature = "alloc")]
crate::impl_cdef![<T: ConstDefault> Self::new() => RcWeak<T>];

#[doc(inline)]
pub use crate::Sized;
