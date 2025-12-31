// devela_base_core::text::fmt::debug
//
//! Defines [`DebugExt`].
//

use crate::{FmtResult, Formatter};

#[doc = crate::_TAG_FMT!()]
#[doc = crate::_TAG_DEBUG!()]
/// Extension for contextual debugging.
///
/// Types implementing this trait support formatting with a caller-supplied context.
/// The context is defined per-type via the `Ctx` associated type, allowing each type
/// to expose whatever formatting modes it needs without affecting other types.
///
/// # Example
/// ```
/// # use devela_base_core::{DebugExt, FmtResult, Formatter};
/// enum Mode { Hex, Dec }
/// struct Value(u32);
/// impl DebugExt for Value {
///     type Ctx = Mode;
///     fn fmt_with(&self, f: &mut Formatter, mode: &Self::Ctx) -> FmtResult<()> {
///         match mode {
///             Mode::Hex => write!(f, "0x{:X}", self.0),
///             Mode::Dec => write!(f, "{}", self.0),
///         }
///     }
/// }
/// let v = Value(255);
/// // caller chooses representation:
/// // v.fmt_with(f, &Mode::Hex) → "0xFF"
/// // v.fmt_with(f, &Mode::Dec) → "255"
/// ```
pub trait DebugExt {
    /// The context used to guide formatting.
    type Ctx;

    /// Formats `self` under the given context.
    fn fmt_with(&self, f: &mut Formatter, ctx: &Self::Ctx) -> FmtResult<()>;
}
