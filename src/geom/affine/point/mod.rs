// devela/src/geom/affine/point/mod.rs
//
//! Positions in affine space without metric or unit semantics
//

mod define; // Point*

#[cfg(feature = "alg")]
mod vector; // impl vector ops
mod turn; // Turn-related impls

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::{Point, Point2d, Point3d},
        };
    }
}
