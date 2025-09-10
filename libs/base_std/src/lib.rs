// devela_base_std::lib
//
//! …
//

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(base_safe, forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_base_std;

#[cfg(feature = "std")]
items! {
    pub mod build;
    pub mod code;
    pub mod data;
    pub mod phys;
    pub mod sys;
    pub mod text;
    pub mod work;
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use all::*;
pub mod all {
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::{
        build::_all::*,
        code::_all::*,
        data::_all::*,
        phys::_all::*,
        sys::_all::*,
        text::_all::*,
        work::_all::*,
    };
}

#[allow(unused_imports)]
#[doc(hidden)] #[rustfmt::skip]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use devela_base_alloc::all::*;
    pub use devela_base_alloc::_workspace_internals::*;
}
