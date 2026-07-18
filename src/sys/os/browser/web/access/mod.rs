// devela/src/sys/os/browser/web/access/mod.rs
//
//! Permissions, credentials, clipboard authority.
//

mod permission; // WebPermission, WebPermissionSet
mod snapshot; // WebPermissionSnapshot

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            permission::*,
            snapshot::*,
        };
    }
}
