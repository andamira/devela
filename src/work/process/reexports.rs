// devela::work::process::reexports
//
//! Reexported items.
//
// Note that std's standalone functions are namespaced in `ExtProcess`.
//
// WAIT: [exit_status_error](https://github.com/rust-lang/rust/issues/84908)

// from workspace base
crate::_reexport_from!(std
    "../../../libs/base_std/src/work/process/reexports.rs", _s);
