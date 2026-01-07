// devela_base_core::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_DOC_NUM_MODULES!()]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num, simd)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_NUM_MODULES =
    crate::_doc!(modules: crate; num: error, geom, niche, quant, wide); // logic, ord, rand
}

mod _internals; // impl_ops!

mod bit; // BitOps, Bitwise
mod cast; // Cast
mod float; // Float, FloatConst, f[32|64]_bits, fsize
mod int; // Divisor, GcdReturn, [i|u]size_[down|up]
mod logic; // ConstBool, False, True, const_bool!
mod traits; // NumConst

pub mod error; // error types
pub mod geom;
pub mod niche; // NonZero*, NonZero*, NonValue*|NonExtreme*, ne!, nz!
pub mod ord; // Cmp
pub mod quant; // Cycle*, Interval, interval!, Ratio
pub mod wide; // define_lane!

crate::structural_mods! { //_mods, _pub_mods, _crate_internals, _workspace_internals
    _mods {
        pub use super::{
            bit::_all::*,
            cast::_all::*,
            float::_all::*,
            int::_all::*,
            logic::_all::*,
            traits::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::error::*;
        pub use super::{
            geom::_all::*,
            niche::_all::*,
            ord::_all::*,
            quant::_all::*,
            wide::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_NUM_MODULES;
    }
    _workspace_internals {
        pub use super::{
            _internals::*,
            float::_workspace_internals::*,
            geom::_workspace_internals::*,
            niche::_workspace_internals::*,
        };
        pub use super::wide::_hidden::*;
    }
}
