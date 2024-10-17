// devela::rend::color
//
//! Chromatic functionality.
//

mod error;
mod r#struct;
mod r#trait;
#[allow(unused_imports)]
pub use {error::*, r#struct::*, r#trait::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{error::*, r#struct::*, r#trait::*};
}
