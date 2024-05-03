// devela::sys::log
//
//!
//

mod logging; // `Logging` namespace
mod reexports; // items from the `log` crate
mod simple; // `LoggerSimple` logger

#[allow(unused_imports)]
pub use {logging::*, reexports::*, simple::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{logging::*, reexports::*, simple::*};
}
