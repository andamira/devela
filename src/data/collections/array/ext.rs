// devela::data::array::ext
//
//!
//

use core::fmt;

/// A formatting wrapper for [arrays][array], implementing `Display` and `Debug`.
#[repr(transparent)]
pub struct ArrayFmt<'a, T: ExtArray>(&'a T);

// Private traits for arrays with elements that implement Debug or Display.
trait ArrayDisplay: ExtArray {
    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

// this one is a bit redundant since arrays of any size can impl Debug,
// nevertheless it's better if we can offer the same api in both cases.
trait ArrayDebug: ExtArray {
    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: ArrayDisplay> fmt::Display for ArrayFmt<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_display(f)
    }
}
impl<T: ArrayDebug> fmt::Debug for ArrayFmt<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_debug(f)
    }
}

// Marker trait to prevent downstream implementations of the `ExtArray` trait.
trait Sealed {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}

/// Extension trait providing convenience methods for [`[T; N]`][array] arrays.
///
/// This trait is sealed and cannot be implemented for any other type.
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtArray: Sealed {
    /// The length of this array.
    const LEN: usize;

    /// Wraps the array in a [`ArrayFmt`] for formatting purposes.
    #[rustfmt::skip]
    fn fmt(&self) -> ArrayFmt<Self> where Self: Sized { ArrayFmt(self) }
}

impl<T, const LEN: usize> ExtArray for [T; LEN] {
    const LEN: usize = LEN;
}

impl<T, const LEN: usize> ArrayDisplay for [T; LEN]
where
    T: fmt::Display,
{
    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (index, element) in self.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            fmt::Display::fmt(element, f)?;
        }
        write!(f, "]")
    }
}
impl<T, const LEN: usize> ArrayDebug for [T; LEN]
where
    T: fmt::Debug,
{
    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
