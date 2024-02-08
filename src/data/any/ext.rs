// devela::data::any::ext
//
//!
//
// MAYBE:
// - WAIT: (const) [type_name](https://github.com/rust-lang/rust/issues/63084)
// - WAIT: [TypeId equality unsoundness](https://github.com/rust-lang/rust/issues/97156)

use core::any::{type_name_of_val, Any, TypeId};

#[cfg(feature = "alloc")]
use crate::_deps::alloc::boxed::Box;

// Marker trait to prevent downstream implementations of the `ExtAny` trait.
mod private {
    pub trait Sealed {}
}
impl<T: Any> private::Sealed for T {}
impl<T: Any> ExtAny for T {}

/// Extension trait providing convenience methods for `T: Any`.
///
/// This trait is sealed and cannot be implemented manually.
#[rustfmt::skip]
pub trait ExtAny: Any + private::Sealed {

    /* type */

    /// Returns the `TypeId` of `self`.
    ///
    /// # Examples
    /// ```
    /// use devela::data::any::{ExtAny, TypeId};
    ///
    ///  let x = 5;
    ///  assert_eq!(x.type_of(), TypeId::of::<i32>());
    /// ```
    #[inline]
    #[must_use]
    fn type_of(&self) -> TypeId { TypeId::of::<Self>() }

    /// Returns the type name of `self`.
    ///
    /// # Examples
    /// ```
    /// use devela::data::any::ExtAny;
    ///
    /// let x = 5;
    /// assert_eq!(x.type_name(), "i32");
    /// ```
    #[inline]
    #[must_use]
    fn type_name(&self) -> &'static str { type_name_of_val(self) }

    /// Returns `true` if `Self` is of type `T`.
    ///
    /// # Examples
    /// ```
    /// use devela::data::any::ExtAny;
    ///
    /// let val = 5;
    /// assert!(val.type_is::<i32>());
    /// assert!(!val.type_is::<u32>());
    ///
    /// // Compared to Any::is():
    /// let any = val.as_any_ref();
    /// // assert!(any.type_is::<i32>()); // doesn't work for &dyn Any
    /// // assert!(val.is::<i32>()); // doesn't work for T: Any
    /// assert!(any.is::<i32>()); // does work for &dyn Any
    /// ```
    #[inline]
    #[must_use]
    fn type_is<T: 'static>(&self) -> bool { self.type_id() == TypeId::of::<T>() }

    /* upcasts */

    /// Upcasts `&self` as `&dyn Any`.
    ///
    /// # Examples
    /// ```
    /// use devela::data::any::{Any, ExtAny};
    ///
    /// let val = 5;
    /// let any: &dyn Any = &val as &dyn Any;
    /// assert!(any.is::<i32>()); // works direcly for dyn Any
    /// ```
    #[inline]
    #[must_use]
    fn as_any_ref(&self) -> &dyn Any where Self: Sized { self }

    /// Upcasts `&mut self` as `&mut dyn Any`.
    ///
    /// # Examples
    /// ```
    /// use devela::data::any::{Any, ExtAny};
    ///
    /// let mut x = 5;
    /// let any: &mut dyn Any = x.as_any_mut();
    /// assert!(any.is::<i32>());
    /// ```
    #[inline]
    #[must_use]
    fn as_any_mut(&mut self) -> &mut dyn Any where Self: Sized { self }

    /// Upcasts `Box<self>` as `Box<dyn Any>`.
    ///
    /// # Examples
    /// ```
    /// use devela::data::any::{Any, ExtAny};
    ///
    /// let x = Box::new(5);
    /// let any: Box<dyn Any> = x.as_any_box();
    /// assert!(any.is::<i32>());
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    fn as_any_box(self: Box<Self>) ->  Box<dyn Any> where Self: Sized { self }
}
