// devela::num::algebra::linear
//
//! Linear algebra.
//

// mod matrix;
mod vector;

// pub use matrix::*;
pub use vector::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::vector::*;
}
