// devela::num::unit
//
//! Unit prefixes.
//

mod helpers; // impl_try_from!

mod bi; // UnitBi
mod si; // UnitSi
mod traits; // Unit

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{bi::*, si::*, traits::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
