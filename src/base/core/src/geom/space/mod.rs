// devela_base_core::geom::space
//
#![doc = crate::_DOC_GEOM_SPACE!()] // public
#![doc = crate::_doc!(modules: crate::geom; space)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

// pub mod field; // Scalar and vector quantities defined over space.
// pub mod grid; // Geometric grid systems using guides, modules, and spatial rhythm.
// pub mod layout; // Spatial constraint-based layout and region resolution.
// pub mod motion; // Geometric motion and change of space over time.
// pub mod part; // Spatial partitioning, subdivision, and tessellation.
// pub mod topol; // Topological properties of space such as connectivity and boundaries.

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     field::_all::*,
        //     grid::_all::*,
        //     layout::_all::*,
        //     motion::_all::*,
        //     part::_all::*,
        //     topol::_all::*,
        // };
    }
}
