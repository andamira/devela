// devela::code::asserts
//
//! Asserts
//

mod dynamic;
#[allow(unused_imports)]
pub use dynamic::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::dynamic::*;
}
