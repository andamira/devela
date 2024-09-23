// devela::code::asserts::static
//
//! Static assertions.
//

mod r#const;
#[allow(unused_imports)]
pub use r#const::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::r#const::*;
}
