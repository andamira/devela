// devela::sys::log
//
//!
//

mod logging;
mod reexports;
mod simple;

#[allow(unused)]
pub use {logging::*, reexports::*, simple::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{logging::*, reexports::*, simple::*};
}
