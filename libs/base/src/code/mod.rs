// devela_base::code
//
#![doc = crate::_DOC_CODE!()]
//

mod any; // dynamic typing and reflection
mod reexports;

pub mod error; // general errors definitions
pub mod marker; // core::marker, type_marker!, type_resource!, TypeResource, TypeResourced
pub mod ops; // ::core::ops::*
pub mod result; // utility macros and functions
pub mod util; // utility macros and functions

util::_structural::structural_mods! { // _mods, _pub_mods, _workspace_internals
    _mods {
        pub use super::{any::*, reexports::*};
    }
    _pub_mods {
        pub use super::{
            error::_all::*, marker::_all::*, ops::_all::*, result::_all::*, util::_all::*,
        };
    }
    _workspace_internals {
        pub/*workspace*/ use super::util::_workspace_internals::*;
    }
}
