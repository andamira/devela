// devela::os::term::ansi
//
//! ANSI codes.
//

#![allow(non_snake_case)]

mod codes;
mod color;
mod macros;

#[cfg(feature = "std")]
mod print;

// re-export private sub-modules
pub use {codes::*, color::*, macros::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{codes::*, color::*, macros::*};
}
