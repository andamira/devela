// devela::codegen::reexport
//
//! Reexported items.
//

#[cfg(any(feature = "depend", feature = "devela_macros"))]
/// <span class="stab portability" title="re-exported from the `devela_macros`
/// crate">`devela_macros`</span>
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "codegen", feature = "depend")))
)]
pub use crate::depend::devela_macros::{cif, compile, compile_attr};

/// <span class="stab portability" title="re-exported from the `paste crate`">`paste`</span>
pub use super::paste::paste;
