// devela::string::egc
//
//! Extended grapheme cluster.
//

mod array_string;
mod non_nul;
#[cfg(feature = "alloc")]
mod string;
mod r#trait;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array_string::*, non_nul::*, r#trait::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::string::*;
}
