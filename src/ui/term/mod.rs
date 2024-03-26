// devela::ui::term
//
//! Terminal functionality.
//

// safety:
#![cfg_attr(feature = "safe_ui_term", forbid(unsafe_code))]

mod ansi;
mod cli;

#[allow(unused_imports)]
pub use {ansi::*, cli::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ansi::*, cli::*};
}
