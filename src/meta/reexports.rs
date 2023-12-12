// devela::meta::reexport
//
//! Reexported items.
//

/// <span class="stab portability" title="re-exported from `devela_macros`
/// crate">`devela_macros`</span>
#[cfg_attr(feature = "nightly", doc(cfg(feature = "meta")))]
pub use devela_macros::{cif, coalesce, compile, compile_attr};

/// <span class="stab portability" title="re-exported from the `paste crate`">`paste`</span>
pub use super::paste::paste;
