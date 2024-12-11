// devela::sys::mem::size
//
//! Memory size functionality.
//

mod byte;
mod expr;

#[cfg(feature = "bit")]
mod bit;

crate::items! { // structural access: doc_inline, items_hidden, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        #[doc(inline)]
        pub use super::{byte::*, expr::size_of_expr};

        #[doc(inline)]
        #[cfg(feature = "bit")]
        pub use super::bit::*;
    }
    pub(super) mod items_hidden {
        #[doc(hidden)]
        pub use super::expr::__size_of_expr;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
