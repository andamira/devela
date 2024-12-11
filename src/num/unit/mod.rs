// devela::num::unit
//
//! Unit prefixes.
//

mod helpers; // impl_try_from!

mod bi; // UnitBi
mod si; // UnitSi
mod traits; // Unit

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{bi::*, si::*, traits::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
