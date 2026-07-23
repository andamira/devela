// devela/src/geom/affine/mod.rs
//
#![doc = crate::_DOC_GEOM_AFFINE!()] // public
#![doc = crate::_doc!(modules: crate::geom; affine)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod define;
mod facet;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::Simplex,
            facet::{SimplexFacetView, SimplexFacetIter},
        };
    }
}
