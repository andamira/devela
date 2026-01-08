// devela::ui::front::term::ansi::print
//
//! Defines [`ansi_print`].
//!
//! The function depends on:
//! - both `linux` and `unsafe_syscall` features enabled.
//! - or on the `std` feature enabled, otherwise.
//

#![allow(unused, reason = "± std or linux")]

use devela::{IoResult, IoWrite};

crate::CONST! {
    DOC_ANSI_PRINT = "
It abstracts away specific backend implementations.

# Example
```
# use devela::{Ansi, AnsiColor3, ansi_print};
ansi_print(&Ansi::ERASE_SCREEN_B);
ansi_print(&Ansi::CURSOR_MOVE3_B(120, 80));
ansi_print(&Ansi::COLORS_BRIGHT_BG_B(AnsiColor3::Blue, AnsiColor3::Black));
```
See also the [`ansi!`][crate::ansi] macro.

";
}

/* print methods */
// In sync with /src/sys/os/print/mod.rs

// std version (overrides linux)
#[doc = crate::_tags!(term platform)]
/// A function to print an ANSI escape `sequence` of bytes to `stdout`
#[doc = crate::_doc_location!("ui/front/term")]
#[doc = DOC_ANSI_PRINT!()]
#[cfg_attr(
    nightly_doc,
    doc(cfg(any(feature = "std", all(feature = "linux", feature = "unsafe_syscall"))))
)]
#[cfg(feature = "std")]
// #[cfg(not(all(feature = "linux", feature = "unsafe_syscall", not(miri), any_target_arch_linux)))]
pub fn ansi_print(sequence: &[u8]) -> IoResult<()> {
    crate::Io::stdout().write_all(sequence)
}

// linux version (only if not(std)) (because of the extra conversions)
#[doc = crate::_tags!(term platform)]
/// A function to print an ANSI escape `sequence` of bytes to `stdout`
#[doc = crate::_doc_location!("ui/front/term")]
#[doc = DOC_ANSI_PRINT!()]
#[cfg_attr(
    nightly_doc,
    doc(cfg(any(feature = "std", all(feature = "linux", feature = "unsafe_syscall"))))
)]
#[cfg(not(feature = "std"))]
#[cfg(all(feature = "linux", feature = "unsafe_syscall", not(miri), any_target_arch_linux))]
pub fn ansi_print(sequence: &[u8]) -> IoResult<()> {
    crate::Linux::print_bytes(sequence).map_err(crate::LinuxError::to_io)
}

#[doc = crate::_tags!(term platform)]
/// The most efficient print method, exclusive for `std`.
#[doc = crate::_doc_location!("ui/front/term")]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
pub fn ansi_print_std(sequence: &[u8]) -> IoResult<()> {
    crate::Io::stdout().write_all(sequence)
}

#[doc = crate::_tags!(term linux)]
/// The most efficient print method, exclusive for `linux`.
#[doc = crate::_doc_location!("ui/front/term")]
///
/// This method avoids having to perform extra error conversions.
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "linux", feature = "unsafe_syscall"))))]
#[cfg(all(feature = "linux", feature = "unsafe_syscall", not(miri), any_target_arch_linux))]
pub fn ansi_print_linux(sequence: &[u8]) -> crate::LinuxResult<()> {
    crate::Linux::print_bytes(sequence)
}
