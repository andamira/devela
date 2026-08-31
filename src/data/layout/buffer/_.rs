// devela/src/data/layout/buffer/_.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_BUFFER!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; buffer)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod_ linear; // buffer_linear!
    mod_ ring; // buffer_ring!
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            linear::_all::*,
            ring::_all::*,
        };
    }
    _hidden {
        pub use super::{
            linear::_hidden::*,
            ring::_hidden::*,
        };
    }
}
