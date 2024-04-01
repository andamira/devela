// devela::sys::log
//
//!
//

mod reexports;
mod simple;

#[allow(unused)]
pub use {reexports::*, simple::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{reexports::*, simple::*};
}
