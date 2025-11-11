// devela::code::result::reexports

use crate::{ConstDefault, mod_path};

mod_path!(+pub _c "../../../libs/base_core/src/code/result/reexports.rs");

#[doc(inline)]
pub use devela_base_core::code::result::{
    Chain, Hook, Mismatch, OptRes, OptResExt, OptionExt, OptionFmt, OptionFmtOr, OptionFmtOrElse,
    Own, ResultExt, serr, sok, unwrap,
};

impl<N: ConstDefault, H: ConstDefault> ConstDefault for Mismatch<N, H> {
    /// Returns a *const* default `Mismatch`.
    const DEFAULT: Self = Self { need: N::DEFAULT, have: H::DEFAULT, info: "" };
}
