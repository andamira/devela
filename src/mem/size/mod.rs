// devela::mem::size
//
//! Memory size functionality.
//

#[allow(unused_imports)]
use crate::code::items;

mod byte;
mod expr;
#[allow(unused_imports)]
pub use {byte::*, expr::*};

#[cfg(feature = "mem_bit")]
items! { mod bit; pub use bit::*; }

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{byte::*, expr::size_of_expr};

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "mem_bit")]
    pub use super::bit::*;
}
