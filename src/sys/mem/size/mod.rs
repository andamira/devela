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
    #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline { #[doc(inline)]
        pub use super::{byte::*, expr::size_of_expr};
        #[cfg(feature = "bit")]
        pub use super::bit::*;
        #[cfg(feature = "alloc")] pub use super::heap::*; // WIP
    }
    pub(super) mod items_hidden { #[doc(hidden)]
        pub use super::expr::__size_of_expr;
    }
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
