// devela::mem::size
//
//! Functionality related to memory size.
//

/* contains always compiled items */

mod expr;
mod size;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub use {expr::*, size::*};

/* feature-gated */

#[cfg(feature = "mem")]
mod bit;

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {bit::*, expr::*, size::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::size::*;

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::{bit::*, expr::mem_size_of_expr};
}
