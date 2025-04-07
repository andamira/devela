// devela::media::color::rgb::impls
//
//!
//

mod helpers; // helper fns

mod u8;
mod u8alpha;
// mod u16;
// mod u16alpha;

#[cfg(feature = "_float_f32")]
crate::items! { mod f32; mod f32alpha; }
#[cfg(feature = "_float_f64")]
crate::items! { mod f64; mod f64alpha; }

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub(crate) use super::helpers::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
