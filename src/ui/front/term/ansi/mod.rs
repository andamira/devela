// devela::ui::front::term::ansi
//
//! ANSI codes.
//

#![allow(non_snake_case)]

mod codes;
mod color;

// re-export private sub-modules
#[allow(unused_imports)]
pub use {codes::*, color::*};

pub(super) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{codes::*, color::*};
}
