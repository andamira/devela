//
#![doc = crate::_DOC_NUM_FIN_BIT!()] // private
#![doc = crate::_doc!(modules: crate::num; bit)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod _benches;
    mod _docs;

    #[cfg(feature = "bitwise")]
    mod ops; // BitOps
    mod span; // BitSpan
    mod_ wise; // Bitwise, bitwise!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            span::*,
            wise::_all::*,
        };
        #[cfg(feature = "bitwise")]
        pub use super::ops::BitOps;
    }
    _hidden {
        pub use super::wise::_hidden::*;
    }
}
