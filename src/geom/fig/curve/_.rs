// devela/src/geom/fig/curve/_.rs
//
#![doc = crate::_DOC_GEOM_FIG_CURVE!()] // public
#![doc = crate::_doc!(modules: crate::geom::fig; curve)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod arc;
    mod_ bezier;
    // mod_ line; // Linear geometric primitives such as segments and rays
    mod_ path;
    mod_ spline;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            arc::*,
            bezier::_all::*,
            line::_all::*,
            path::_all::*,
            spline::_all::*,
        };
    }
}
