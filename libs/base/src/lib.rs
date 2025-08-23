// devela_base::lib
//
//!
//

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_base;

pub mod code;
pub mod num;
pub mod text;

#[doc(hidden)]
pub use all::*;
pub mod all {
    // public items, feature-gated, visible at their origin and here in `all`
    //
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        code::_all::*,
        num::_all::*,
        text::_all::*,
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use _workspace_private::*;
#[doc(hidden)] #[rustfmt::skip]
pub/*workspace*/ mod _workspace_private {
    #[allow(unused_imports)]
    pub/*workspace*/ use super::{
        code::_workspace_private::*,
    };
}
