// devela/src/sys/os/term/ansi/macro/_.rs
//
//! Defines the [`ansi!`] macro.
//
// NOTES:
// - features are in sync with /src/sys/os/print/mod.rs.
// - different macros are necessary to avoid evaluating the feature-bounds on user time.
// - versions differ only in having support for printing, and in the Ansi print method called.

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod _docs; // _DOC_ANSI
    mod define; // _ansi_define
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::ansi;
    }
    _crate_internals {
        pub(crate) use super::define::_ansi_define;
    }
}

crate::_linux_syscall! { _ansi_define!(print ansi_print_linux); }
crate::_std_not_linux_syscall! { _ansi_define!(print ansi_print_std); }
crate::_not_std_or_linux_syscall! { _ansi_define!(fallback); }
