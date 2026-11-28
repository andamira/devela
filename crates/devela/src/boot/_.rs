//
#![doc = crate::boot::_DOC_BOOT!()]
#![doc = concat![crate::boot::_DOC_BOOT_MODULES!(), " [◉](mod@crate::all_::_boot)"]]
//!
//! Available independently of the other devela modules,
//! including in `no_std` environments without allocation.
//

#![allow(unused_imports)]

#[path = "_doc/_.rs"]
mod _doc; // documentation
pub(crate) mod _policy; // workspace policy
#[doc(hidden)]
pub mod dep; // dependencies
#[path = "expr/_.rs"]
pub mod expr; // elementary expression
mod init; // ConstInit
#[path = "rust/_.rs"]
pub mod rust; // re-exports from Rust lib
#[path = "synth/_.rs"]
pub mod synth; // structural synthesis

pub(crate) mod _strap {
    #[doc(hidden, no_inline)]
    pub use super::{_doc::_all::*, synth::_all::*};
}

synth::mods_out! {
    _mods {
        pub use super::{
            _doc::_all::*,
            init::ConstInit,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            expr::_all::*,
            rust::*,
            synth::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            expr::{is, unwrap, whilst},
            synth::CONST,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _doc::_crate_internals::*,
            init::_impl_init,
            synth::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::expr::_hidden::*;
        #[doc(hidden)]
        pub use super::_policy::{_devela_policy, __devela_unreachable_unchecked};
    }
}
