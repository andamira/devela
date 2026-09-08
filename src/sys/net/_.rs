// devela/src/sys/net/_.rs
//
#![doc = crate::_DOC_SYS_NET!()] // public
#![doc = crate::_doc!(modules: crate::sys; net: inet, transport)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
        mod _reexport_core;

    pub mod_ inet;
        // mod_ link;
        // mod_ mesh;
        // mod_ name;
        // mod_ secure;
    pub mod_ transport;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        pub use super::{
            inet::_all::*,
            // link::_all::*,
            // mesh::_all::*,
            // name::_all::*,
            // secure::_all::*,
            transport::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
