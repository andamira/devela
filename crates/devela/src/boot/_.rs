//
#![doc = crate::boot::_DOC_BOOT!()]
#![doc = concat![crate::boot::_DOC_BOOT_MODULES!(), " [◉](mod@crate::all_::_boot)"]]
//!
//! The small vocabulary and machinery from which devela's modules are bootstrapped.
//!
//! Available independently of the other devela modules,
//! including in `no_std` environments without allocation.
//

#![allow(unused_imports)]

#[path = "_doc/_.rs"]
mod _doc; // documentation
pub(crate) mod _policy; // workspace policy

#[path = "base/_.rs"]
pub mod base; // root-independent devela vocabulary
#[doc(hidden)]
pub mod deps; // dependencies
#[path = "meta/_.rs"]
pub mod meta; // structural metaprogramming
#[path = "rust/_.rs"]
pub mod rust; // re-exports from Rust lib

pub(crate) mod _strap {
    #[doc(hidden, no_inline)]
    pub use super::{_doc::_all::*, meta::_all::*};
}

meta::mods_out! {
    _mods {
        pub use super::_doc::_all::*;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            base::_all::*,
            meta::_all::*,
            rust::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            base::code::{ConstInit, is, unwrap, whilst},
            meta::CONST,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _doc::_crate_internals::*,
            base::_impl_init,
            meta::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::base::_hidden::*;
        #[doc(hidden)]
        pub use super::_policy::{_devela_policy, __devela_unreachable_unchecked};
    }
}
