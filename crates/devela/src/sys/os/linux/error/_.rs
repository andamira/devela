//
//! Linux error handling ABI and typed representations.
//

crate::mods_in! {
    mod consts; // LINUX_ERRNO, LINUX_EXIT
    mod error; // LinuxError, LinuxResult
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            consts::*,
            error::*,
        };
    }
}
