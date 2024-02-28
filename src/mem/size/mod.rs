// devela::mem::size
//
//! Memory size functionality.
//

/* always compiled, non-public modules */
mod bit;
mod byte;
mod expr;
mod reexports;

pub use {bit::*, byte::*, expr::*, reexports::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{bit::*, byte::*, expr::mem_size_of_expr, reexports::*};
}
