//
#![doc = crate::_DOC_DEVICE!()] // public
#![doc = crate::_doc!(modules: crate; devic)] // …
#![doc = crate::_doc!(flat:"device")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ audio;
    // mod_ comm;
    // mod_ display;
    // mod_ input;
    // mod_ motion;
    // mod_ power;
    // mod_ sensor;
    // mod_ vision;
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     audio::_all::*,
        //     comm::_all::*,
        //     display::_all::*,
        //     input::_all::*,
        //     motion::_all::*,
        //     power::_all::*,
        //     sensor::_all::*,
        //     vision::_all::*,
        // };
    }
}
