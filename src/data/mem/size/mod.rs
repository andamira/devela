// devela::data::mem::size
//
//! Memory size functionality.
//

mod byte;
mod expr;
mod reexports;
#[allow(unused_imports)]
pub use {byte::*, expr::*, reexports::*};

#[cfg(feature = "data_bit")]
mod bit;
#[cfg(feature = "data_bit")]
pub use bit::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{byte::*, expr::mem_size_of_expr, reexports::*};

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "data_bit")]
    pub use super::bit::*;
}
