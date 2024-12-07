// devela::sys::mem::size
//
//! Memory size functionality.
//

mod byte;
mod expr;

#[cfg(feature = "bit")]
mod bit;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{byte::*, expr::size_of_expr};
        #[cfg(feature = "bit")]
        pub use super::bit::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
