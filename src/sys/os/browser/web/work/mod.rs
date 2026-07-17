// devela/src/sys/os/browser/web/work/mod.rs
//
//!
//

// mod clock;
// mod frame;
// mod schedule;
mod time; // impls for JsInstant and JsTimeout
mod worker; // WebWorker, WebWorkerError, WebWorkerJob

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            worker::*,
        };
    }
}
