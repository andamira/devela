// devela/src/text/str/namespace/_.rs
//
//! Defines the [`Str`] namespace.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Str

    // impls
    mod utf8_traversal; // UTF-8 conversion and traversal
    mod boundary; // equality and boundary
    mod writing; // writing, transliteration and repetition
    mod range;
    mod take;
    mod split;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::Str;
    }
}
