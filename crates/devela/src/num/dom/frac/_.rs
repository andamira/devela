//
#![doc = crate::_DOC_NUM_DOM_FRAC!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; frac)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
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
