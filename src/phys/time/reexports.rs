// devela::phys::time::reexports
//
//!
//

crate::mod_path!(+pub _c "../../../libs/base_core/src/phys/time/reexports.rs");
crate::mod_path!(std +pub _s "../../../libs/base_std/src/phys/time/reexports.rs");

#[doc(inline)]
pub use devela_base_core::Timeout;

#[cfg(feature = "std")]
pub use devela_base_std::{SystemTimeError, TimeError};
