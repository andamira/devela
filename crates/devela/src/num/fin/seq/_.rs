//
//! Constructive discrete sequences.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;
    #[cfg(feature = "_docs_examples")]
    mod _example; // SeqExample

    mod define; // seq!
    mod _internal; // __seq!
    mod_ impls;
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::seq;
        #[cfg(feature = "_docs_examples")]
        pub use super::_example::SeqExample;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _internal::__seq,
            impls::_hidden::*,
        };
    }
}
