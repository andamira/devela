// devela::text::ascii
//
//! ASCII strings and characters.
#![doc = crate::doc_!(extends: ascii)]
#![doc = crate::doc_!(modules: crate::text; ascii)]
#![doc = crate::doc_!(newline)]
//!
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
