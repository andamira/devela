// devela/src/vita/_.rs
//
#![doc = crate::_DOC_VITA!()] // public, root
#![doc = crate::_DOC_VITA_MODULES!()]
#![doc = crate::_doc!(flat:"vita")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_vita", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_VITA_MODULES =
    crate::_doc!(modules: crate; vita: play); // body, craft, home, love, mind
}

crate::mods_in! {
    // pub mod_ body; // Embodied capability, health, and physical limits of living beings.
    // pub mod_ craft; // Practical, learned ways of shaping material reality to support life.
    // pub mod_ home; // Inhabited space, from dwelling to built and shared environments.
    // pub mod_ love; // Relational life, from kinship and care to intimacy and community.
    // pub mod_ mind; // Cognition, memory, meaning, and inner orientation of lived experience.
    pub mod_ play; // Expression, play, and shared enjoyment beyond necessity or survival.
}
crate::mods_out! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            // body::_all::*,
            // craft::_all::*,
            // home::_all::*,
            // love::_all::*,
            // mind::_all::*,
            play::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_VITA_MODULES;
    }
}
