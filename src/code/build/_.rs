// devela/src/code/build/_.rs
//
#![doc = crate::_DOC_CODE_BUILD!()] // private
#![doc = crate::_doc!(modules: crate::code; build)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "std")]
    mod namespace; // Build
    // mod _util;
}
crate::mods_out! { // _mods
    _mods {
    #[cfg(feature = "std")]
        pub use super::namespace::Build;
    }
}
