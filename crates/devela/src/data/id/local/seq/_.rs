//
//!
//

crate::mods_in! {
    #[cfg(all(test, any(feature = "dep_portable_atomic", target_has_atomic = "8")))]
    mod _test;
    #[cfg(all(
        any(test, feature = "_docs_examples"),
        any(feature = "dep_portable_atomic", target_has_atomic = "64"),
    ))]
    mod _example; // IdSeqU64Example

    mod define; // id_seq!
    mod _internal; // __id_seq!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::define::id_seq;

        #[cfg(all(
            any(test, feature = "_docs_examples"),
            any(feature = "dep_portable_atomic", target_has_atomic = "64"),
        ))]
        pub use super::_example::IdSeqU64Example;
    }
    _hidden {
        pub use super::_internal::__id_seq;
    }
}
