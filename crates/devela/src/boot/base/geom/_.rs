//
//!
//

#[path = "dir/_.rs"]
mod dir;

crate::boot::meta::mods_out! { // _mods
    _mods {
        pub use super::dir::_all::*;
    }
}
