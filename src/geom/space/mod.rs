// devela/src/geom/space/mod.rs
//
#![doc = crate::_DOC_GEOM_SPACE!()] // public
#![doc = crate::_doc!(modules: crate::geom; space)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

// mod earth; // Terrestrial reference systems, geodesy, projections, and surveying
// mod field; // Quantities defined over geometric domains and their level sets
// mod grid; // Regular spatial frames, guides, modules, and geometric lattices
// mod layout; // Constraint-driven placement and resolution of spatial elements
// mod motion; // Geometric trajectories, velocities, and change through time
// mod part; // Decomposition, covering, subdivision, and tessellation of space
mod topol; // Incidence, orientation, adjacency, connectivity, and boundaries

crate::structural_mods! { // _mods
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
