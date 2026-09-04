// devela/src/num/dom/frac/_.rs
//
//! Fractional functionality.
//

crate::mods_in! {
    #[cfg(feature = "int")]
    mod_ wrapper;

    // mod r#trait;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "int")]
        pub use super::wrapper::_all::*;

        // pub use super::r#trait::*;
    }
}
