// devela::mem::size
//
//! Memory size functionality.
//

/* modules */

// always compiled, non-public
mod bit;
mod reexports;
mod size;

// feature-gated, non-public
#[cfg(feature = "mem")]
mod expr;

/* re-exports */

// always compiled, non-public
pub use {bit::*, reexports::*, size::*};

// feature-gated, non-public
#[cfg(feature = "mem")]
pub use expr::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{bit::*, reexports::*, size::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::expr::mem_size_of_expr;
}
