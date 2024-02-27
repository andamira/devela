// devela::mem::size
//
//! Memory size functionality.
//

/* always compiled, non-public modules */
mod bit;
mod expr;
mod reexports;
mod size;

pub use {bit::*, expr::mem_size_of_expr, reexports::*, size::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{bit::*, expr::mem_size_of_expr, reexports::*, size::*};
}
