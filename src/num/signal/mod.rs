// devela/src/num/signal/mod.rs
//
#![doc = crate::_DOC_NUM_SIGNAL!()] // public
#![doc = crate::_doc!(modules: crate::num; signal)] // …
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

// mod curve; // Curves, envelopes, and interpolated forms
// mod emit; // Signal emitters and produced streams
// mod filter; // Domain-generic filtering and smoothing
// mod phase; // Phase, cycles, and periodic progression
// mod sample; // Sampling, stepping, and discrete realization
// mod transform; // Signal mapping, modulation, and transformation
// mod weave; // Signal weaving, composition, and interconnection

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        // pub use super::{
        //     // curve::_all::*,
        //     // emit::_all::*,
        //     // filter::_all::*,
        //     // sample::_all::*,
        //     // transform::_all::*,
        //     // weave::_all::*,
        // };
    }
    _pub_mods {
        // pub use super::{
        // };
    }
}
