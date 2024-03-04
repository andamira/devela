// devela::data::collections::array::d1
//
//! 1-dimensional array
//

/* always compiled */

// without re-exports
mod impl_traits;
mod methods;

mod definitions;
pub use definitions::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;
}
