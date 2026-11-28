//
//!
//

mod read_at;
mod write_at;

crate::boot::meta::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            read_at::read_at,
            write_at::write_at,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::write_at::__unicode_scalar_write_utf8_at;
    }
}
