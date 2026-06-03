// devela::sys::os::term::ansi::macro
//
//! Defines the [`ansi!`] macro.
//
// NOTES:
// - features are in sync with /src/sys/os/print/mod.rs.
// - different macros are necessary to avoid evaluating the feature-bounds on user time.
// - versions differ only in having support for printing, and in the Ansi print method called.

#[cfg(test)]
mod tests;

mod _docs; // _DOC_ANSI
mod define; // _ansi_define

crate::_linux_syscall! { _ansi_define!(print ansi_print_linux); }
crate::_std_not_linux_syscall! { _ansi_define!(print ansi_print_std); }
crate::_not_std_or_linux_syscall! { _ansi_define!(fallback); }

crate::structural_mods! {
    _mods {
        pub use super::ansi;
    }
    _crate_internals {
        pub(crate) use super::define::_ansi_define;
    }
}
