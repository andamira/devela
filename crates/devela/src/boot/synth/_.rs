//! Structural synthesis and crate construction.

mod _reexport;
mod r#const;
mod items;
mod mods_out;

pub(crate) mod _all {
    pub use super::{r#const::CONST, items::items, mods_out::mods_out};
}
pub(crate) mod _crate_internals {
    pub use super::_reexport::_reexport;
}
