// devela::code::result::reexports

use crate::{ConstDefault, mod_path};

mod_path!(+pub _c "../../../libs/base/src/code/result/reexports.rs");

pub use devela_base::code::result::{Chain, Hook, Mismatch, Own, ValueQuant};

impl<N: ConstDefault, H: ConstDefault> ConstDefault for Mismatch<N, H> {
    /// Returns a *const* default `Mismatch`.
    const DEFAULT: Self = Self { need: N::DEFAULT, have: H::DEFAULT, info: "" };
}
