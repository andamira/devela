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

// #[cfg(feature = "devela_mem")] // THINK rename: size?
// mod impl_mem {
//     use devela::mem::BitSize;
//
//     bit_size![= 0; for Ansi];
//     bit_size![= 3; for AnsiColor3b];
//     bit_size![= 8; for AnsiColor8b];
//
//
//     impl BitSize<0> for Ansi {}
//     impl BitSize<3> for AnsiColor3b {}
//     impl BitSize<8> for AnsiColor8b {}
// }
