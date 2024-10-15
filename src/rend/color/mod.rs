// devela::rend::color
//
//! Chromatic functionality.
//

mod r#struct;
mod r#trait;
#[allow(unused_imports)]
pub use {r#struct::*, r#trait::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#struct::*, r#trait::*};
}
