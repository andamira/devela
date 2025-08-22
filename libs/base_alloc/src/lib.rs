// devela_base_alloc::lib
//
//!
//

#![no_std]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_base_alloc;

#[doc(hidden)]
#[allow(unused_imports)]
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
    // };
}
