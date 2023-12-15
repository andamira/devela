// devela::mem::size
//
//! Memory size functionality.
//

/* contains always compiled items */

mod bit;
mod reexports;
mod size;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub use {bit::*, reexports::*, size::*};

/* feature-gated */

#[cfg(feature = "mem")]
mod expr;

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {bit::*, expr::*, reexports::*, size::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{bit::*, reexports::*, size::*};

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::expr::mem_size_of_expr;
}
