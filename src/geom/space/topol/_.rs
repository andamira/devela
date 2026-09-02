// devela/src/geom/space/topol/_.rs
//
#![doc = crate::_DOC_GEOM_SPACE_TOPOL!()] // public
#![doc = crate::_doc!(modules: crate::geom::space; topol)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod boundary;
    // mod adjacency;
    mod point_segment;
    // mod relation; // MAYBE
    mod turn;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // boundary::*,
            // adjacency::*,
            point_segment::PointSegmentRelation,
            turn::Turn,
        };
    }
}
