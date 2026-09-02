// devela/src/geom/space/_.rs
//
#![doc = crate::_DOC_GEOM_SPACE!()] // public
#![doc = crate::_doc!(modules: crate::geom; space)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ earth; // Terrestrial reference systems, geodesy, projections, and surveying
    // mod_ field; // Quantities defined over geometric domains and their level sets
    // mod_ grid; // Regular spatial frames, guides, modules, and geometric lattices
    // mod_ layout; // Constraint-driven placement and resolution of spatial elements
    // mod_ motion; // Geometric trajectories, velocities, and change through time
    // mod_ part; // Decomposition, covering, subdivision, and tessellation of space
    mod_ topol; // Incidence, orientation, adjacency, connectivity, and boundaries
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // earth::_all::*,
            // field::_all::*,
            // grid::_all::*,
            // layout::_all::*,
            // motion::_all::*,
            // part::_all::*,
            topol::_all::*,
        };
    }
}
