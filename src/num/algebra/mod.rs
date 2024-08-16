// devela::num::algebra
//
//! Linear algebra and symbolic computation.
//

mod linear;
// mod symbolic;

pub use linear::*;
// pub use symbolic::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::linear::all::*;
}
