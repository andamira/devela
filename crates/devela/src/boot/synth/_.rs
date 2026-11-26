//
//! Structural synthesis of Rust code.
//

mod _reexport;
mod r#const;
mod items;
mod mods_out;

mods_out::mods_out! {
    _mods {
        pub use super::{r#const::CONST, items::items, mods_out::mods_out};

        #[doc = crate::_tags!(code procedural_macro)]
        pub use devela_macros::{cif, compile, compile_attr, mods_in, paste};
    }
    _crate_internals {
        pub(crate) use super::_reexport::_reexport;
    }
}
