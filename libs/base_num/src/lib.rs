// devela_base_num::lib
//
//!
//

#![cfg_attr(not(feature = "__std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_base_num;

mod _internals; // upcasted_op!, impl_ops!

mod int; // Int
mod float; // Float, FloatConst
mod quant; // ValueQuant

structural_mods! { // _mods
    _mods {
        pub use super::{
            float::_all::*,
            int::_all::*,
            quant::_all::*,
        };
    }
}

#[doc(hidden)]
pub use all::*;
pub mod all {
    #![allow(unused_imports)]
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        int::_all::*,
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use _crate_internals::*;
#[doc(hidden)]
mod _crate_internals {
    #[doc(hidden)]
    #[rustfmt::skip]
    pub use {
        devela_base_core::_workspace_internals::*,
        devela_base_core::all::*,
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use _workspace_internals::*;
#[doc(hidden)]
pub mod _workspace_internals {
    #[doc(hidden)]
    #[rustfmt::skip]
    #[allow(unused_imports)]
    pub use super::{
        _internals::*,
        float::_workspace_internals::*,
        int::_workspace_internals::*,
    };
}
