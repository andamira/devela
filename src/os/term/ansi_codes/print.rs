// devela::os::term::ansi::print
//
//! ANSI print method.
//

#[cfg(feature = "std")]
use std::io::{stdout, Write};

#[cfg(any(
    feature = "std",
    all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "linux",
        feature = "unsafe_os",
        not(miri),
    )
))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "linux", feature = "unsafe_os"))))
)]
/// # Print method
impl super::Ansi {
    /// Convenience method to print an ANSI escape `sequence` of bytes to `stdout`.
    ///
    /// It uses the `Write` trait on `std` and
    /// [`linux_print_bytes`][crate::os::linux::linux_print_bytes] on `no_std`.
    ///
    /// # Example
    /// ```
    /// use devela::os::term::{Ansi, AnsiColor3};
    ///
    /// Ansi::print(&Ansi::ERASE_SCREEN);
    /// Ansi::print(&Ansi::CURSOR_MOVE3(120, 80));
    /// Ansi::print(&Ansi::COLORS_BRIGHT_BG(AnsiColor3::Blue, AnsiColor3::Black));
    /// ```
    #[inline]
    // MAYBE -> Result<()>
    pub fn print(sequence: &[u8]) {
        #[cfg(feature = "std")]
        let _ = stdout().write_all(sequence);

        #[cfg(all(
            not(feature = "std"),
            feature = "linux",
            feature = "unsafe_os",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            )
        ))]
        crate::os::linux::linux_print_bytes(sequence);
    }
}
