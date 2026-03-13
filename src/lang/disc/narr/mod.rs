// devela::lang::disc::narr
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG_DISC_NARR!()] // public
#![doc = crate::_doc!(modules: crate::lang::disc; narr)]
// choice, beat, dialog, guard, link, passage, route, scene
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod choice;  // explicit choices, options, and player-facing branches
// mod beat;    // narrative beats, pacing shifts, and delivery markers
// mod dialog;  // dialogue turns, attributed lines, and exchanges
// mod guard;   // narrative guards, conditions, and availability gates
// mod link;    // links, transitions, and narrative targets
// mod passage; // passages, lexia, and authored text units
// mod route;   // routes, paths, outcomes, and ending lines
// mod scene;   // scenes, situations, and local narrative contexts

crate::structural_mods! { // _mods, _reexports
    _mods {
        // pub use super::{
        //     choice::_all::*,
        //     beat::_all::*,
        //     dialog::_all::*,
        //     guard::_all::*,
        //     link::_all::*,
        //     passage::_all::*,
        //     route::_all::*,
        //     scene::_all::*,
        // };
    }
    _reexports {
        // pub use devela_base_core::lang::disc::narr::{
        // };
    }
}
