// devela_base_core::geom::affine
//
#![doc = crate::_DOC_GEOM_AFFINE!()] // public
#![doc = crate::_doc!(modules: crate::geom; affine)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

// mod frame; // Affine reference frames and coordinate systems.
// mod map; // Affine maps combining linear transformation and translation.
mod point; // Positions in affine space without metric or unit semantics.
// mod transform; // Semantic affine transformations applied to geometric entities.

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // frame::_all::*,
            // map::_all::*,
            point::*,
            // transform::_all::*,
        };
    }
    _reexports {
        // use devela_base_core::geom::affine::{
        // };
    }
}
