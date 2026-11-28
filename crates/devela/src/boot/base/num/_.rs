//
//!
//

#[path = "fin/_.rs"]
mod fin;
#[path = "quant/_.rs"]
mod quant;

crate::boot::meta::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{fin::bitwise, quant::Sign};
    }
    _hidden {
        #[doc(hidden)]
        pub use super::fin::{__bitwise, __bitwise_backend, __bitwise_inline, __bitwise_methods};
    }
}
