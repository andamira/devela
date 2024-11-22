// devela::num::cmp
//
//! Comparing and ordering values.
// #![doc = crate::doc_!(extends: cmp)]
// #![doc = crate::doc_!(modules: crate::num; cmp)]
// #![doc = crate::doc_!(newline)]
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(_cmp_·)]
crate::items! {
    mod compare; // `Compare`
    pub use compare::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[cfg(_cmp_·)]
    pub use super::compare::*;
}
