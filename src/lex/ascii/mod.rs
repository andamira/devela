// devela::lex::ascii
//
//! ASCII strings and characters, extends `std::`[`ascii`].
//!
//! [`ascii`]: std::ascii
//

mod char;
mod wrapper;

#[allow(unused_imports)]
pub use {char::AsciiChar, wrapper::Ascii};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{char::AsciiChar, wrapper::Ascii};
}
