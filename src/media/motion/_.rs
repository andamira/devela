// devela/src/media/motion/_.rs
//
#![doc = crate::_DOC_MEDIA_MOTION!()] // public
#![doc = crate::_doc!(modules: crate::media; motion)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    // mod animation;   // composition of timed changes
    // mod object;      // primary motion

    // mod camera;      // secondary/view motion
    // mod curve;       // easing, envelopes, splines

    // mod gesture;     // expressive motion primitives
    // mod rhythm;      // pulses, meters, cycles, trigger grids

    // mod time;        // normalized time, phase, progress
    // mod transform;   // translation, rotation, scale
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     object::*,
        //     camera::*,
        // };
    }
}
