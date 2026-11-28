//
//! Foundational devela vocabulary, grouped by its semantic homes.
//!
//! These are devela-owned definitions and operations that must remain available
//! independently of semantic root-module selection. Their canonical semantic
//! paths continue to live in the corresponding root modules.
//

#[path = "code/_.rs"]
pub mod code;
#[path = "data/_.rs"]
pub mod data;
#[path = "geom/_.rs"]
pub mod geom;
#[path = "num/_.rs"]
pub mod num;

crate::boot::meta::mods_out! { // _pub_mods, _crate_internals, _hidden
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            code::_all::*,
            data::_all::*,
            geom::_all::*,
            num::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::code::_crate_internals::*;
    }
    _hidden {
        pub use super::{
            data::_hidden::*,
            num::_hidden::*,
        };
    }
}
