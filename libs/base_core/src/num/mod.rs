// devela_base_core::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_doc!(modules: crate; num: error, geom, niche, quant)] // logic, ord, rand
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]

mod _internals; // impl_ops!

mod cast; // Cast
mod float; // Float, FloatConst, f[32|64]_bits, fsize
mod int; // Divisor, GcdReturn, [i|u]size_[down|up]
mod logic; // ConstBool, False, True, const_bool!
mod ord; // Cmp

pub mod error; // error types
pub mod geom;
pub mod niche; // NonZero*, NonZero*, NonValue*|NonExtreme*, ne!, nz!
pub mod quant; // Cycle*, Interval, interval!, Ratio

crate::structural_mods! { //_mods, _pub_mods
    _mods {
        pub use super::{
            cast::_all::*,
            float::_all::*,
            int::_all::*,
            logic::_all::*,
            ord::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::error::*;
        pub use super::{
            geom::_all::*,
            niche::_all::*,
            quant::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            _internals::*,
            float::_workspace_internals::*,
            geom::_workspace_internals::*,
            niche::_workspace_internals::*,
        };
    }
}
