// devela::num::algebra
//
//! Linear algebra and symbolic computation.
//

pub mod linear;
#[doc(hidden)]
#[allow(unused_imports)]
pub use linear::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::linear::all::*;
}
