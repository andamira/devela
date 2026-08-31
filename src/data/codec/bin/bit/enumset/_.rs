// devela/src/data/codec/bin/bit/enumset/_.rs
//
//! An enum with an associated bit set.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    #[cfg(any(test, feature = "_docs_examples"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values ("_docs_examples")))))]
    // WAIT 1.99: #[crate::macro_apply(crate::__doc_auto_hide_features((("_docs_examples"))))]
    mod _example; // EnumExample, EnumSetExample

    mod define; // enumset!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            define::enumset,
        };
        #[cfg(feature = "_docs_examples")]
        pub use super::_example::*;
    }
    _hidden {
        pub use super::define::{
            __enumset_impl_enum_blocks,
            __enumset_impl_unit_iter,
            __enumset_to_set_pattern,
        };
    }
}
