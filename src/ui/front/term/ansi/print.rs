// devela::ui::front::term::ansi::print
//
//! Implements [`Ansi::print`].
//!
//! The method depends on:
//! - both `linux` and `unsafe_syscall` features enabled.
//! - or on the `std` feature enabled, otherwise.
//

#![allow(unused, reason = "± std or linux")]

use devela::{IoResult, IoWrite};

crate::CONST! {
    DOC_ANSI_PRINT = "Common method to print an ANSI escape `sequence` of bytes to `stdout`.

It abstracts away specific backend implementations.

# Example
```
# use devela::{Ansi, AnsiColor3b};
Ansi::print(&Ansi::ERASE_SCREEN);
Ansi::print(&Ansi::CURSOR_MOVE3(120, 80));
Ansi::print(&Ansi::COLORS_BRIGHT_BG(AnsiColor3b::Blue, AnsiColor3b::Black));
```";
}

/// # Print method
//
// In sync with /src/sys/os/print/mod.rs:
#[cfg_attr(
    nightly_doc,
    doc(cfg(any(feature = "std", all(feature = "linux", feature = "unsafe_syscall"),)))
)]
impl super::Ansi {
    // std version (even if linux)
    #[doc = DOC_ANSI_PRINT!()]
    #[cfg(feature = "std")]
    // #[cfg(not(all(feature = "linux", feature = "unsafe_syscall", not(miri),
    //     any( // targets:
    //         target_arch = "x86", target_arch = "x86_64",
    //         target_arch = "arm", target_arch = "aarch64",
    //         target_arch = "riscv32", target_arch = "riscv64"
    //     ),
    // )))]
    pub fn print(sequence: &[u8]) -> IoResult<()> {
        crate::Io::stdout().write_all(sequence)
    }

    // linux version (only if not(std)) (because of the extra conversions)
    #[doc = DOC_ANSI_PRINT!()]
    #[cfg(not(feature = "std"))]
    #[cfg(all(feature = "linux", feature = "unsafe_syscall", not(miri),
        any( // targets:
            target_arch = "x86", target_arch = "x86_64",
            target_arch = "arm", target_arch = "aarch64",
            target_arch = "riscv32", target_arch = "riscv64"
        ),
    ))]
    pub fn print(sequence: &[u8]) -> IoResult<()> {
        crate::Linux::print_bytes(sequence).map_err(crate::LinuxError::to_io)
    }

    /// The most efficient print method, exclusive for `std`.
    #[cfg(feature = "std")]
    pub fn print_std(sequence: &[u8]) -> IoResult<()> {
        crate::Io::stdout().write_all(sequence)
    }

    /// The most efficient print method, exclusive for `linux`.
    ///
    /// This method avoids having to perform extra error conversions.
    #[cfg(all(feature = "linux", feature = "unsafe_syscall", not(miri),
        any( // targets:
            target_arch = "x86", target_arch = "x86_64",
            target_arch = "arm", target_arch = "aarch64",
            target_arch = "riscv32", target_arch = "riscv64"
        ),
    ))]
    pub fn print_linux(sequence: &[u8]) -> crate::LinuxResult<()> {
        crate::Linux::print_bytes(sequence)
    }
}
