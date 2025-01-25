// devela::data::list::array::ext
//
//!
//

use crate::{Debug, Display, FmtResult, Formatter};

/// A formatting wrapper for core [arrays][array], implementing [`Display`] and [`Debug`].
///
/// It is created by the [`ExtArray::fmt`] method.
#[repr(transparent)]
pub struct ArrayFmt<'a, T: ExtArray>(&'a T);

/// Private trait for arrays with elements that implement [`Display`].
trait ArrayDisplay: ExtArray {
    fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()>;
}

/// Private trait for arrays with elements that implement [`Debug`].
///
/// This trait is a bit redundant since arrays of any size can impl `Debug`,
/// nevertheless it's better if we can offer the same api in both cases.
trait ArrayDebug: ExtArray {
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()>;
}

impl<T: ArrayDisplay> Display for ArrayFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
        self.0.fmt_display(f)
    }
}
impl<T: ArrayDebug> Debug for ArrayFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
        self.0.fmt_debug(f)
    }
}

/// Marker trait to prevent downstream implementations of the [`ExtArray`] trait.
trait Sealed {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}

/// Extension trait providing convenience methods for [`[T; N]`][array] arrays.
///
/// This trait is sealed and cannot be implemented for any other type.
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtArray: Sealed {
    /// The length of this array.
    const LEN: usize;

    /// Wraps the array in an [`ArrayFmt`] for formatting purposes.
    #[rustfmt::skip]
    fn fmt(&self) -> ArrayFmt<Self> where Self: Sized { ArrayFmt(self) }
}

impl<T, const LEN: usize> ExtArray for [T; LEN] {
    const LEN: usize = LEN;
}

impl<T: Display, const LEN: usize> ArrayDisplay for [T; LEN] {
    fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()> {
        write!(f, "[")?;
        for (index, element) in self.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            Display::fmt(element, f)?;
        }
        write!(f, "]")
    }
}
impl<T: Debug, const LEN: usize> ArrayDebug for [T; LEN] {
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()> {
        f.debug_list().entries(self.iter()).finish()
    }
}
