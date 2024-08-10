// devela::code::any::ext
//
//!
//
// MAYBE:
// - WAIT: (const) [type_name](https://github.com/rust-lang/rust/issues/63084)
// - WAIT: [TypeId equality unsoundness](https://github.com/rust-lang/rust/issues/97156)
// - WAIT: [trait_upcasting](https://github.com/rust-lang/rust/issues/65991)

use core::any::{type_name_of_val, Any, TypeId};

#[cfg(feature = "alloc")]
use crate::_liballoc::boxed::Box;

// Marker trait to prevent downstream implementations of the `ExtAny` trait.
mod private {
    pub trait Sealed {}
}
impl<T: ?Sized + Any> private::Sealed for T {}
impl<T: ?Sized + Any> ExtAny for T {}

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
    /// use devela::code::{ExtAny, TypeId};
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
    /// use devela::code::ExtAny;
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
    /// use devela::code::ExtAny;
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
    /// use devela::code::{Any, ExtAny};
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
    /// use devela::code::{Any, ExtAny};
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
    /// use devela::code::{Any, ExtAny};
    ///
    /// let x = Box::new(5);
    /// let any: Box<dyn Any> = x.as_any_box();
    /// assert!(any.is::<i32>());
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    fn as_any_box(self: Box<Self>) ->  Box<dyn Any> where Self: Sized { self }

    /* downcasts */

    /// Returns some shared reference to the inner value if it is of type `T`.
    ///
    /// This method is only needed when not dealing directly with `dyn Any` trait objects,
    /// since it's [already implemented for `dyn Any`](Any#method.downcast_ref).
    ///
    /// # Examples
    /// ```
    /// use core::fmt::Display;
    /// use devela::{Any, DstArray, DstStack, DstValue, ExtAny};
    ///
    /// trait Trait: Any + Display {}
    /// impl Trait for i32 {}
    /// impl Trait for char {}
    /// impl Trait for bool {}
    ///
    /// # #[cfg(feature = "alloc")]
    /// // in the heap:
    /// {
    ///     # use devela::all::{Box, Vec};
    ///     let b: Box<dyn Trait> = Box::new(5);
    ///     if let Some(n) = (*b).downcast_ref::<i32>() {
    ///         assert_eq![n, &5_i32];
    ///     }
    ///
    ///     let bb: Vec<Box<dyn Trait>> = vec![Box::new(true), Box::new(6), Box::new('c')];
    ///     for b in bb {
    ///         if let Some(n) = (*b).downcast_ref::<i32>() {
    ///             assert_eq![n, &6_i32];
    ///         }
    ///     }
    /// }
    /// // in the stack:
    /// {
    ///     let v = DstValue::<dyn Trait, DstArray<usize, 2>>::new(7, |v| v as _).unwrap();
    ///     if let Some(n) = (*v).downcast_ref::<i32>() {
    ///         assert_eq![n, &7_i32];
    ///     }
    ///
    ///     let mut vv = DstStack::<dyn Trait, DstArray<u32, 12>>::new();
    ///     vv.push(true, |v| v).unwrap();
    ///     vv.push(8_i32, |v| v).unwrap();
    ///     vv.push('c', |v| v).unwrap();
    ///     for v in vv.iter() {
    ///         if let Some(n) = (*v).downcast_ref::<i32>() {
    ///             assert_eq![n, &8_i32];
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "unsafe_layout")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        // SAFETY: We verify T is of the right type before downcasting
        unsafe { (*self).type_is::<T>().then(|| &*<*const _>::cast(self)) }
    }

    /// Returns some exclusive reference to the inner value if it is of type `T`.
    ///
    /// This method is only needed when not dealing directly with `dyn Any` trait objects,
    /// since it's [already implemented for `dyn Any`][Any#method.downcast_mut].
    #[inline]
    #[must_use]
    #[cfg(feature = "unsafe_layout")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        // SAFETY: We verify T is of the right type before downcasting
        unsafe { (*self).type_is::<T>().then(|| &mut *<*mut _>::cast(self)) }
    }
}
