// devela::num
//
//! Numerical types and math operations.
#![doc = crate::doc_!(modules: crate; num: algebra, logic, niche, rand, wave)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: cmp, num)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod _private; // upcasted_op!, impl_ops!

mod cmp; // Compare
mod error; // NumError, NumResult
mod float; // fsize, ExtFloat, ExtFloatConst, Float
mod frac; // Frac
mod int; // [i|u]size_[down|up], Int
mod interval; // Interval
mod no; // NoNum
mod primitive; // Cast, Primitive[Cast|Join|Split]
mod sign; // Sign
mod traits; // Num, NumRef
mod unit; // Unit, Unit[Bi|Si]

pub mod algebra;
pub mod logic;
pub mod niche;

#[cfg(feature = "geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "geom")))]
pub mod geom;
#[cfg(feature = "rand")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
pub mod rand;
#[cfg(feature = "wave")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "wave")))]
pub mod wave;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_hidden::*, doc_inline::*, items_private::*};

    mod doc_inline {
        #[allow(unused_imports, reason = "frac")]
        pub use super::{
            cmp::all::*, error::*, float::all::*, frac::all::*, int::all::*,
            interval::*, no::*, primitive::all::*, sign::*, traits::*,
            unit::all::*,
        };
        #[cfg(feature = "geom")]
        pub use super::geom::all::*;
        #[cfg(feature = "rand")]
        pub use super::rand::all::*;
        #[cfg(feature = "wave")]
        pub use super::wave::all::*;
    }
    mod doc_hidden { #[doc(hidden)] #[doc(no_inline)]
        pub use super::{algebra::all::*, logic::all::*, niche::all::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{cmp::always::*, float::always::*, niche::always::*};
    }
    pub(super) mod items_private { #![allow(unused_imports)]
        pub(crate) use super::_private::*;
    }
}
