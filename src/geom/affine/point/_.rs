// devela/src/geom/affine/point/_.rs
//
//! Positions in affine space without metric or unit semantics
//

crate::mods_in! {
    mod define; // Point*

    #[cfg(feature = "alg")]
    mod vector; // impl vector ops
    mod_ turn; // Turn-related impls
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::{Point, Point2, Point3},
        };
    }
}
