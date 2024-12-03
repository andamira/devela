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

#[cfg(feature = "bit")]
items! { mod bit; pub use bit::*; }

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{byte::*, expr::size_of_expr};

    #[doc(inline)]
    #[cfg(feature = "bit")]
    pub use super::bit::*;
}
