// devela/src/num/alg/vector/mod.rs
//
//! Linear algebra vectors.
//!
//! Vectors represent the difference between two positions.
//!
//! They are characterized by their *direction* and *magnitude*, and
//! their direction can be decomposed into *orientation* and *sense*.
//

#[cfg(test)]
mod _test;

mod define; //

mod methods;
mod ops;

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::{Vector, Vector2d, Vector3d},
        };
    }
}
