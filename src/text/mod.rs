// devela::text
//
//! Text manipulation, extends
//! `std::{`[`ascii`][std::ascii],
//! [`char`][std::char],
//! [`fmt`][std::fmt],
//! [`str`][std::str],
//! [`string`][std::string]`}`.
//

/* contains always compiled items */

pub mod ascii;
pub mod char;

#[allow(unused)]
#[cfg(not(feature = "text"))]
pub(crate) use {ascii::*, char::*};

/* feature-gated */

// public modules
#[cfg(feature = "text")]
pub mod egc;
#[cfg(feature = "text")]
pub mod fmt;

// private modules
#[cfg(feature = "text")]
mod array_string;
#[cfg(feature = "text")]
mod error;
#[cfg(feature = "text")]
mod ext;
#[cfg(feature = "text")]
mod helpers;
#[cfg(feature = "text")]
mod non_nul;
#[cfg(feature = "text")]
mod reexports;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "text")]
pub use {ascii::all::*, char::all::*, egc::all::*, fmt::all::*};

// re-export private sub-modules
#[cfg(feature = "text")]
pub use {array_string::*, error::*, ext::*, non_nul::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ascii::all::*, char::all::*};

    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::{
        array_string::*, egc::all::*, error::*, ext::*, fmt::all::*, non_nul::*, reexports::*,
    };
}
