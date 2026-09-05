// devela/src/sys/os/browser/web/access/_.rs
//
//! Permissions, credentials, clipboard authority.
//

crate::mods_in! {
    mod permission; // WebPermission, WebPermissionSet
    mod snapshot; // WebPermissionSnapshot
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            permission::*,
            snapshot::*,
        };
    }
}
