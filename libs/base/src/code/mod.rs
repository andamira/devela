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

crate::items! { // structural access: _mods, _pub_mods, _workspace_private, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
        pub use super::{any::*, reexports::*};
    }
    mod _pub_mods {
        pub use super::{
            error::_all::*, marker::_all::*, ops::_all::*, result::_all::*, util::_all::*,
        };
    }
    pub(super) mod _workspace_private { #[allow(unused_imports)]
        pub/*workspace*/ use super::util::_workspace_private::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
