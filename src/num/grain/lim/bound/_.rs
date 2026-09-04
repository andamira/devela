// devela/src/num/grain/lim/bound/_.rs
//
//! Bounded numeric carriers and range-preserving arithmetic.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    #[cfg(any(test, feature = "_docs_examples"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("_docs_examples")))))]
    mod _example; // BoundI8Example, (BoundI8SymExample)

    mod define; // bound_int!
    mod signed;
    // mod unsigned;
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            define::bound_int,
        };

        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _crate_internals {
        #[cfg(any(test, feature = "_docs_examples"))]
        pub(crate) use super::_example::*;
    }
    _hidden {
        pub use super::{
            signed::__bound_int_impl_signed,
            // unsigned::__bound_int_impl_unsigned,
        };
    }
}
