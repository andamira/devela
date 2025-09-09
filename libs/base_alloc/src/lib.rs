// devela_base_alloc::lib
//
//! …
//

#![no_std]
#![cfg_attr(base_safe, forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate alloc;
extern crate self as devela_base_alloc;

pub mod data;
pub mod lang;
pub mod num;
pub mod sys;
pub mod text;

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
    pub use super::{
        data::_all::*,
        lang::_all::*,
        num::_all::*,
        sys::_all::*,
        text::_all::*,
    };
}

#[allow(unused_imports)]
#[doc(hidden)] #[rustfmt::skip]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use devela_base_core::all::*;
    pub use devela_base_core::_workspace_internals::*;

    pub use devela_base_num::all::*;
    pub use devela_base_num::_workspace_internals::*;
}
