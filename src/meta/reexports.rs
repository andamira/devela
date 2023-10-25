// devela::meta::reexport
//
//! Reexported items.
//

#[cfg(any(feature = "dep", feature = "devela_macros"))]
/// <span class="stab portability" title="re-exported from `devela_macros`
/// (which can be enabled instead of `depend`">`devela_macros`</span>
#[cfg_attr(feature = "nightly", doc(cfg(all(feature = "meta", feature = "dep"))))]
pub use crate::_dep::devela_macros::{cif, compile, compile_attr};

/// <span class="stab portability" title="re-exported from the `paste crate`">`paste`</span>
pub use super::paste::paste;
