// devela::data::mem::size
//
//! Memory size functionality.
//

/* always compiled */

mod bit;
mod byte;
mod expr;
mod reexports;
#[allow(unused_imports)]
pub use {bit::*, byte::*, expr::*, reexports::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{bit::*, byte::*, expr::mem_size_of_expr, reexports::*};
}
