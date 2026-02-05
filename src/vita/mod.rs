// devela::vita
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_VITA!()] // public, root
#![doc = crate::_DOC_VITA_MODULES!()]
#![doc = crate::_doc!(flat:"vita")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_vita", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_VITA_MODULES =
    crate::_doc!(modules: crate; vita: _); // body, craft, home, love, mind, play
}

// pub mod body; // Embodied capability, health, and physical limits of living beings.
// pub mod craft; // Practical, learned ways of shaping material reality to support life.
// pub mod home; // Inhabited space, from dwelling to built and shared environments.
// pub mod love; // Relational life, from kinship and care to intimacy and community.
// pub mod mind; // Cognition, memory, meaning, and inner orientation of lived experience.
// pub mod play; // Expression, play, and shared enjoyment beyond necessity or survival.

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        // pub use super::{
        //     body::_all::*,
        //     craft::_all::*,
        //     home::_all::*,
        //     love::_all::*,
        //     mind::_all::*,
        //     play::_all::*,
        // };
    }
    _crate_internals {
        pub(crate) use super::_DOC_VITA_MODULES;
    }
}
