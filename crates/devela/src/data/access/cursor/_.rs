//
#![doc = crate::_DOC_DATA_ACCESS_CURSOR!()] // private
#![doc = crate::_doc!(modules: crate::data::access; cursor)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "byte")]
    mod_ byte; // ByteCursor, ByteCursorError
    // mod traits;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "byte")]
        pub use super::byte::_all::*;
        // pub use super::traits::*;
    }
}
