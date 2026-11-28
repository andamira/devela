//
//!
//

mod init;
#[path = "ops/_.rs"]
mod ops;
#[path = "result/_.rs"]
mod result;
#[path = "util/_.rs"]
mod util;

crate::boot::meta::mods_out! {
    _mods {
        pub use super::{
            init::ConstInit,
            ops::_all::*,
            result::_all::*,
            util::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::init::_impl_init;
    }
}
