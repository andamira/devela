// devela_base_core::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_doc!(modules: crate; num: geom, logic, niche, ord, quant, rand)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]

mod cast; // Cast
mod float;
mod int;
mod logic; // ConstBool, False, True, const_bool!
mod ord;

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
            geom::_workspace_internals::*,
            niche::_workspace_internals::*,
        };
    }
}
