//
//!
//

mod bitwise;

crate::boot::meta::mods_out! { // _mods, _hidden
    _mods {
        pub use super::bitwise::bitwise;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::bitwise::{
            __bitwise, __bitwise_backend, __bitwise_inline, __bitwise_methods,
        };
    }
}
