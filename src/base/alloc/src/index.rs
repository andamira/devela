// devela_base_alloc::index
//
//!
//

#![cfg_attr(not(feature = "__std"), no_std)]
#![cfg_attr(base_safe, forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate alloc;
extern crate self as devela_base_alloc;
macro_rules! __crate_name {
    () => {
        "devela_base_alloc"
    };
}
#[allow(unused_imports)]
pub(crate) use __crate_name;

pub mod code;
pub mod data;
pub mod geom;
pub mod lang;
pub mod media;
pub mod num;
pub mod phys;
pub mod run;
pub mod sys;
pub mod text;
pub mod ui;
pub mod work;
//
mod yard;
#[doc(hidden)]
pub use yard::_dep;
//
// mod _doc;

#[doc(hidden)]
#[allow(unused_imports)]
pub use zall::*;
pub mod zall {
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
        data::_all::*,
        geom::_all::*,
        lang::_all::*,
        media::_all::*,
        num::_all::*,
        phys::_all::*,
        run::_all::*,
        sys::_all::*,
        text::_all::*,
        ui::_all::*,
        work::_all::*,
    };
}

// private, internal items
#[allow(unused_imports)]
pub(crate) use _crate_internals::*;
mod _crate_internals {
    #[rustfmt::skip]
    pub(crate) use super::{
        code::_crate_internals::*,
        data::_crate_internals::*,
        geom::_crate_internals::*,
        lang::_crate_internals::*,
        media::_crate_internals::*,
        num::_crate_internals::*,
        phys::_crate_internals::*,
        run::_crate_internals::*,
        sys::_crate_internals::*,
        text::_crate_internals::*,
        ui::_crate_internals::*,
        work::_crate_internals::*,
    };
}

#[allow(unused_imports)]
#[doc(hidden)] #[rustfmt::skip]
pub use _workspace_internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub mod _workspace_internals {
    #![allow(unused_imports)]
    pub use devela_base_core::zall::*;
    pub use devela_base_core::_workspace_internals::*;
}
