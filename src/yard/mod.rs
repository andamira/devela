// devela::yard
//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
//
#![doc = crate::_tags!(internal)]
#![doc = crate::_DOC_YARD!()] // internal
#![doc = crate::_doc!(modules: crate; yard: _dep)]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_YARD!()]
//

pub mod _dep;

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::yard::*;
        #[doc(inline)]
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::yard::*;
        #[cfg(feature = "std")]
        pub use devela_base_std::yard::*;
    }
}
