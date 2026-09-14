// devela/sys/hw/capture/scan/_.rs
//
//! Scanner device interfaces.
//
// Backends for SANE, eSCL, WIA/TWAIN, and related protocols,
// providing device discovery, capabilities, and raw scan data streams.
//

crate::mods_in! {
    // mod escl;
    // mod sane;
    // mod twain;
    // mod wia;
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     escl::_all::*,
        //     sane::_all::*,
        //     twain::_all::*,
        //     wia::_all::*,
        // };
    }
}
