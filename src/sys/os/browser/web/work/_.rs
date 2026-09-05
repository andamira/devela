// devela/src/sys/os/browser/web/work/_.rs
//
//!
//

crate::mods_in! {
    // mod clock;
    // mod frame;
    // mod schedule;
    mod time; // impls for JsInstant and JsTimeout
    mod worker; // WebWorker, WebWorkerError, WebWorkerJob
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            worker::*,
        };
    }
}
