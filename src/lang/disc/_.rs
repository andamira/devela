// devela/src/lang/disc/_.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG_DISC!()] // public
#![doc = crate::_doc!(modules: crate::lang; disc: narr)]
// act, case, dialog, flow, ifx, lyric, rhetoric, style
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod act; // Discourse moves, intents, prompts, replies
    // mod case; // Argumentative structure, persuasion, fallacies
    // mod dialog; // Dialogue turns, attribution, and exchanges
    // mod flow; // Conditional discourse flow
    // mod journal; // Journalistic inquiry, attribution, reporting, and presentation
    // mod ifx; // Interactive-fiction structure and execution
    // mod lyric; // Lyric expression, voice, felt experience
    mod_ narr; // Narrative structure, scenes, routes
    // mod rhetoric; // Rhetorical devices, tropes, figures
    // mod style; // Stylistic patterns, tone, register
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // act::_all::*,
            // case::_all::*,
            // dialog::_all::*,
            // flow::_all::*,
            // ifx::_all::*,
            // lyric::_all::*,
            narr::_all::*,
            // rhetoric::_all::*,
            // style::_all::*,
        };
    }
}
