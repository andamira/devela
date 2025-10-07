// devela_base_text::lib
//
//!
//

#![cfg_attr(not(feature = "__std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_base_text;

// mod grapheme;

structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     grapheme::_all::*,
        // };
    }
}

#[doc(hidden)]
pub use all::*;
pub mod all {
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    // #[allow(unused_imports)]
    // #[rustfmt::skip]
    // #[doc(inline)]
    // pub use super::{
    //     grapheme::_all::*,
    // };
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
    // #[doc(hidden)]
    // #[rustfmt::skip]
    // #[allow(unused_imports)]
    // pub use super::{
    //     _internals::*,
    //     _?::_workspace_internals::*,
    // };
}
