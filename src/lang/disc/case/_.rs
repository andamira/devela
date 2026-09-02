// devela/src/lang/disc/case/_.rs
//
//! Claims, evidence, inference, objections, and persuasive structure.
//

crate::mods_in! {
    // mod_ appeal;    // Persuasive appeals to values, emotion, trust, or authority
    // mod_ burden;    // Burdens of proof and responsibility for justification
    // mod_ claim;     // Assertions proposed for acceptance or examination
    // mod_ evidence;  // Grounds and observations offered in support of claims
    // mod_ fallacy;   // Invalid or misleading patterns of reasoning
    // mod_ inference; // Steps relating evidence and premises to conclusions
    // mod_ objection; // Challenges, exceptions, and counterarguments
    // mod_ rebuttal;  // Responses to objections and opposing claims
    // mod_ stance;    // Positions held toward claims or disputed questions
    // mod_ warrant;   // Principles connecting evidence to a claim
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // appeal::_all::*,
            // burden::_all::*,
            // claim::_all::*,
            // evidence::_all::*,
            // fallacy::_all::*,
            // inference::_all::*,
            // objection::_all::*,
            // rebuttal::_all::*,
            // stance::_all::*,
            // warrant::_all::*,
        };
    }
}
