// devela::_info
//
//! Extra information about the library.
//

#![cfg(any(doc, test))]
#![cfg_attr(nightly_doc, doc(cfg(any(doc, test))))]

crate::CONST! {
    COMMON_DOC = "\n\nAll items flat re-exported.";
}

crate::items! {
    #[doc = concat![crate::_DOC_CODE!(), COMMON_DOC!()]]
    pub mod _code { #[allow(unused)] pub use super::super::code::_all::*; }
    #[doc = concat![crate::_DOC_DATA!(), COMMON_DOC!()]]
    pub mod _data { #[allow(unused)] pub use super::super::data::_all::*; }
    #[doc = concat![crate::_DOC_GAME!(), COMMON_DOC!()]]
    pub mod _game { #[allow(unused)] pub use super::super::game::_all::*; }
    #[doc = concat![crate::_DOC_LANG!(), COMMON_DOC!()]]
    pub mod _lang { #[allow(unused)] pub use super::super::lang::_all::*; }
    #[doc = concat![crate::_DOC_MEDIA!(), COMMON_DOC!()]]
    pub mod _media { #[allow(unused)] pub use super::super::media::_all::*; }
    #[doc = concat![crate::_DOC_NUM!(), COMMON_DOC!()]]
    pub mod _num { #[allow(unused)] pub use super::super::num::_all::*; }
    #[doc = concat![crate::_DOC_PHYS!(), COMMON_DOC!()]]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }
    #[doc = concat![crate::_DOC_SYS!(), COMMON_DOC!()]]
    pub mod _sys { #[allow(unused)] pub use super::super::sys::_all::*; }
    #[doc = concat![crate::_DOC_TEXT!(), COMMON_DOC!()]]
    pub mod _text { #[allow(unused)] pub use super::super::text::_all::*; }
    #[doc = concat![crate::_DOC_UI!(), COMMON_DOC!()]]
    pub mod _ui { #[allow(unused)] pub use super::super::ui::_all::*; }
    #[doc = concat![crate::_DOC_WORK!(), COMMON_DOC!()]]
    /// <br/><hr>
    pub mod _work { #[allow(unused)] pub use super::super::work::_all::*; }
}

/// Build-time metaprogramming and code generation utilities.
///
/// Contains logic executed during `cargo build`, including:
/// - Feature flag management
/// - Compile-time environment inspection
/// - Procedural code generation
#[cfg(feature = "std")]
#[path = "../../build/main/mod.rs"]
pub mod build;

/// Documented examples.
pub mod examples;

/// Procedural macros.
pub mod macros {
    #![doc = include_str!("../../libs/base_macros/src/Lib.md")]
}

#[cfg(doc)]
crate::items! {
    pub mod features {
        //! Library features.
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./features.md")]
    }
    pub mod nightly {
        //! Nightly features.
        #![doc = include_str!("../../docs/nightly.md")]
    }
    /// Vendored work.
    pub mod vendored;
}
