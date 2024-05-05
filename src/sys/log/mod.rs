// devela::sys::log
//
//!
//

mod config; // `LoggerConfig` struct
mod ext; // `ExtLog` trait
mod logging; // `Logging` namespace
mod print; // `LoggerPrint` logger
mod reexports; // items from the `log` crate

#[allow(unused_imports)]
pub use {config::*, ext::*, logging::*, print::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{config::*, ext::*, logging::*, print::*, reexports::*};
}
