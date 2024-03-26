// devela::ui::term::cli
//
//!
//

#[cfg(all(feature = "std", feature = "crossterm"))]
mod prompt;

#[cfg(all(feature = "std", feature = "crossterm"))]
pub use prompt::*;
