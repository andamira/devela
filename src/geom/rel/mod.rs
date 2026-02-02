// devela::geom::rel
//
#![doc = crate::_DOC_GEOM_REL!()] // public
#![doc = crate::_doc!(modules: crate::geom; rel)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

// mod adjacency; // Neighborhood and reachability relations between entities.
// mod contaiment; // Inside, outside, and inclusion relations.
// mod incidence; // Incidence relations such as touching and crossing.
// mod order; // Ordering relations such as betweenness and sequence.
// mod share; // Relations of shared spatial support or overlap.

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     adjacency::_all::*,
        //     containment::_all::*,
        //     incidence::_all::*,
        //     order::_all::*,
        //     share::_all::*,
        // };
    }
    _reexports {
        // pub use devela_base_core::geom::rel{
        // };
    }
}
