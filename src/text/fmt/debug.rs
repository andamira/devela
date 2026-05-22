// devela::text::fmt::debug
//
//! Defines [`DebugExt`], [`DebugWith`].
//

use crate::{Debug, FmtResult, Formatter};

#[doc = crate::_tags!(fmt debug)]
/// Extension for contextual debugging.
#[doc = crate::_doc_location!("text/fmt")]
///
/// Types implementing this trait support formatting with a caller-supplied context.
/// The context is defined per-type via the `Ctx` associated type, allowing each type
/// to expose whatever formatting modes it needs without affecting other types.
///
/// # Examples
/// ```
/// # use devela::{DebugExt, FmtResult, Formatter};
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
#[rustfmt::skip]
pub trait DebugExt {
    /// The context used to guide formatting.
    type Ctx;
    /// Formats `self` under the given context.
    fn fmt_with(&self, f: &mut Formatter, ctx: &Self::Ctx) -> FmtResult<()>;
    /// Wraps `self` for contextual debug formatting.
    fn debug_with<'a>(&'a self, ctx: &'a Self::Ctx) -> DebugWith<'a, Self> where Self: Sized {
        DebugWith::new(self, ctx)
    }
}

/// A [`Debug`] adapter for formatting a value with a [`DebugExt`] context.
pub struct DebugWith<'a, T: DebugExt + ?Sized> {
    value: &'a T,
    ctx: &'a T::Ctx,
}
impl<'a, T: DebugExt + ?Sized> DebugWith<'a, T> {
    /// Creates a contextual debug adapter.
    pub const fn new(value: &'a T, ctx: &'a T::Ctx) -> Self {
        Self { value, ctx }
    }
}
impl<T: DebugExt + ?Sized> Debug for DebugWith<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        self.value.fmt_with(f, self.ctx)
    }
}
