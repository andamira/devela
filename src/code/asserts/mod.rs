// devela::code::asserts
//
//! Asserts.
//

mod dynamic;
mod r#static;
#[allow(unused_imports)]
pub use {dynamic::*, r#static::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{dynamic::*, r#static::*};
}
