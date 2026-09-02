// devela/src/lang/sem/_.rs
//
#![doc = crate::_DOC_LANG_SEM!()] // public
#![doc = crate::_doc!(modules: crate::lang; sem)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod cue; // Semantic prompts offered for contextual interpretation.
    // mod derive; // Rules, inference, normalization, rewriting
    // mod find; // Matching, selection, and traversal
    // mod interp; // Situated readings and interpretation
    // mod map; // Mappings between semantic regions
    // mod qual; // Semantic qualification
    mod_ rel; // Semantic relations
    // mod schema; // Compile-time semantic vocabulary
    // mod world; // Concrete semantic items and relations
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cue::Cue,
            // derive::_all::*,
            // find::_all::*,
            // interp::_all::*,
            // map::_all::*,
            // qual::_all::*,
            rel::_all::*,
            // schema::_all::*,
            // world::_all::*,
        };
    }
}
