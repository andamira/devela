// devela::sys::os::term::ansi::macro
//
//! Defines the [`ansi!`] macro.
//
// NOTES:
// - features are in sync with /src/sys/os/print/mod.rs.
// - different macros are necessary to avoid evaluating the feature-bounds on user time.
// - versions differ only in having support for printing, and in the Ansi print method called.

crate::macro_apply_alias! {
    linux = #[cfg(all(feature = "_linux_abi", feature = "unsafe_syscall", not(miri),
        any_target_arch_linux))];
    not_linux = #[cfg(not(all(feature = "_linux_abi", feature = "unsafe_syscall", not(miri),
        any_target_arch_linux)))];
}

#[cfg(test)]
mod tests;

mod _docs; // _DOC_ANSI
mod define; // _ansi_define

linux! {
    _ansi_define!(print ansi_print_linux);
}
#[cfg(feature = "std")]
not_linux! {
    _ansi_define!(print ansi_print_std);
}
#[cfg(not(feature = "std"))]
not_linux! {
    _ansi_define!(fallback);
}

crate::structural_mods! {
    _mods {
        pub use super::ansi;
    }
    _crate_internals {
        pub(crate) use super::define::_ansi_define;
    }
}
