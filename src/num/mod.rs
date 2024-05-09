// devela::num
//
//! Numerical types and operations, algebra, geometry, <small>extends
//! `std::{`[`cmp`], [`num`]`}`.</small>
//!
//! [`cmp`]: std::cmp
//! [`num`]: std::num
//

// safety:
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod _private;
#[allow(unused_imports)]
pub(crate) use _private::*;

mod alias;
mod cmp;
mod error;
mod float;
mod no;
mod primitive;
mod sign;
mod r#trait;
pub use {alias::*, cmp::*, error::*, float::*, no::*, primitive::*, r#trait::*, sign::*};

pub mod niche;
pub mod rand;
#[doc(no_inline)]
pub use {niche::all::*, rand::all::*};

#[cfg(_some_int)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_some_int)))]
mod frac;
#[cfg(_some_int)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_some_int)))]
mod int;
#[cfg(_some_int)]
pub use {frac::*, int::*};

#[cfg(feature = "num_geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_geom")))]
pub mod geom;
#[cfg(feature = "num_geom")]
pub use geom::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        alias::*, cmp::all::*, error::*, float::*, niche::all::*, no::*, primitive::*, r#trait::*,
        rand::all::*, sign::*,
    };

    #[doc(inline)]
    #[cfg(feature = "num_geom")]
    #[allow(unused_imports)]
    pub use super::geom::all::*;
    #[doc(inline)]
    #[cfg(_some_int)]
    #[allow(unused_imports)]
    pub use super::{frac::*, int::*};
}
