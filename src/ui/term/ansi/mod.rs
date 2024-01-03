// devela::ui::term::ansi
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
#[allow(unused_imports)]
pub use {codes::*, color::*, macros::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{codes::*, color::*, macros::*};
}
