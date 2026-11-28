//
//!
//

#[path = "access/_.rs"]
mod access;

crate::boot::meta::mods_out! { // _mods, _hidden
    _mods {
        pub use super::access::_all::*;
    }
    _hidden {
        pub use super::access::_hidden::*;
    }
}
