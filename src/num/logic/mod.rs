// devela::num::logic
//
//! Logic related types and functionality.
//

mod bool;
pub use bool::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::bool::*;
}
