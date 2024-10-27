// devela::code::macros::asserts::static
//
//! Static assertions.
//

mod r#const;
mod r#impl;
#[allow(unused_imports)]
pub use {r#const::*, r#impl::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#const::*, r#impl::*};
}
