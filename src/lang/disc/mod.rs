// devela::lang::disc
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG_DISC!()] // public
#![doc = crate::_doc!(modules: crate::lang; disc: narr)]
// argument, lyric, move, plot, rhetoric, style
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod argument; // argumentative structure, persuasion, fallacies
// mod lyric;    // lyric expression, voice, felt experience
// mod move;     // discourse moves, intents, prompts, replies
mod narr; // narrative structure, scenes, routes
// mod plot;     // causal progression, tension, revelation
// mod rhetoric; // rhetorical devices, tropes, figures
// mod style;    // stylistic patterns, tone, register

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // argument::_all::*,
            // lyric::_all::*,
            // move::_all::*,
            narr::_all::*,
            // plot::_all::*,
            // rhetoric::_all::*,
            // style::_all::*,
        };
    }
}
