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
    crate::_doc!(modules: crate; num: dom, error, fin, grain, lin, prob, quant, symb);
}

mod _internals; // impl_ops!, upcasted_op!

mod cast; // Cast
mod float; // Float, FloatConst, f[32|64]_bits, fsize
mod int; // Divisor, GcdReturn, Int, [i|u]size_[down|up]
mod traits; // NumConst

pub mod error; // error types
pub mod fin; // Finite and discrete numeric structures
pub mod niche; // NonZero*, NonZero*, NonValue*|NonExtreme*, ne!, nz!
pub mod quant; // Cycle*, Interval, interval!, Ratio, ValueQuant
// pub mod symb; // WIP
pub mod wide; // define_lane!

crate::structural_mods! { //_mods, _pub_mods, _crate_internals, _workspace_internals, _hidden
    _mods {
        pub use super::{
            cast::_all::*,
            float::_all::*,
            int::_all::*,
            traits::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::error::*;
        pub use super::{
            // WIP
            fin::_all::*,

            niche::_all::*,
            quant::_all::*,
            // symb::_all::*,
            wide::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_NUM_MODULES,
            fin::_crate_internals::*,
        };
    }
    _workspace_internals {
        pub use super::{
            _internals::*,
            float::_workspace_internals::*,
            int::_workspace_internals::*,
            niche::_workspace_internals::*,
        };
    }
    _hidden {
        pub use super::{
            int::_hidden::*,
            wide::_hidden::*,
        };
    }
}
