// devela::phys::time::reexports
//
//!
//

crate::mod_path!(+pub _c "../../../libs/base/src/phys/time/reexports.rs");
crate::mod_path!(std +pub _s "../../../libs/base_std/src/phys/time/reexports.rs");

pub use devela_base::Timeout;

#[cfg(feature = "std")]
pub use devela_base_std::{SystemTimeError, TimeError};
