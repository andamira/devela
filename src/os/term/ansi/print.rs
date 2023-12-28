// devela::os::term::ansi::print
//
//! ANSI print method.
//

use std::io::{stdout, Write};

#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
/// # Print method
impl super::Ansi {
    /// Convenience method to print an ANSI escape `sequence` of bytes to `stdout`.
    ///
    /// It uses the `Write` trait on `std`.
    ///
    /// # Example
    /// ```
    /// use devela::os::term::{Ansi, AnsiColor3b};
    ///
    /// Ansi::print(&Ansi::ERASE_SCREEN);
    /// Ansi::print(&Ansi::CURSOR_MOVE3(120, 80));
    /// Ansi::print(&Ansi::COLORS_BRIGHT_BG(AnsiColor3b::Blue, AnsiColor3b::Black));
    /// ```
    #[inline]
    pub fn print(sequence: &[u8]) -> std::io::Result<()> {
        stdout().write_all(sequence)
    }
}
