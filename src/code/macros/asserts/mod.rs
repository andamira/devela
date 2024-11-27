// devela::code::macros::asserts
//
//! Asserts.
//

mod dynamic;
mod r#static;
#[allow(unused_imports)]
pub use {dynamic::*, r#static::*};

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{dynamic::*, r#static::*};
}
