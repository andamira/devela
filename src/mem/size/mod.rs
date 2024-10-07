// devela::mem::size
//
//! Memory size functionality.
//

use crate::code::items;

mod byte;
mod expr;
mod reexports;
#[allow(unused_imports)]
pub use {byte::*, expr::*, reexports::*};

#[cfg(feature = "mem_bit")]
items! { mod bit; pub use bit::*; }

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{byte::*, expr::mem_size_of_expr, reexports::*};

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "mem_bit")]
    pub use super::bit::*;
}
