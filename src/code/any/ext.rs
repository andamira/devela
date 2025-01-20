// devela::code::any::ext
//
//!
//
// - WAIT: (const) [type_name](https://github.com/rust-lang/rust/issues/63084)
// - WAIT: [trait_upcasting](https://github.com/rust-lang/rust/issues/65991)

use crate::{Any, Hash, Hasher, HasherFx, TypeId};
use core::any::type_name_of_val;

#[cfg(feature = "alloc")]
use crate::Box;

/// Marker trait to prevent downstream implementations of the [`ExtAny`] trait.
trait Sealed {}
impl<T: ?Sized + Any> Sealed for T {}
impl<T: ?Sized + Any> ExtAny for T {}

#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait providing convenience methods for `T:`[`Any`].
///
/// This trait is sealed and cannot be implemented manually.
#[rustfmt::skip]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtAny: Any + Sealed {

    /* type */

    /// Returns the `TypeId` of `Self`.
    ///
    /// # Example
    /// ```
    /// use devela::ExtAny;
    ///
    /// let x = 5;
    /// assert_eq!(x.type_of(), i32::type_id());
    /// ```
    #[must_use]
    fn type_id() -> TypeId { TypeId::of::<Self>() }

    /// Returns the `TypeId` of `self`.
    ///
    /// # Example
    /// ```
    /// use devela::ExtAny;
    ///
    /// let x = 5;
    /// assert_eq!(x.type_of(), i32::type_id());
    /// ```
    #[must_use]
    fn type_of(&self) -> TypeId { TypeId::of::<Self>() }

    /// Returns the type name of `self`.
    ///
    /// # Example
    /// ```
    /// use devela::ExtAny;
    ///
    /// let x = 5;
    /// assert_eq!(x.type_name(), "i32");
    /// ```
    #[must_use]
    fn type_name(&self) -> &'static str { type_name_of_val(self) }

    /// Returns `true` if `Self` is of type `T`.
    ///
    /// # Example
    /// ```
    /// use devela::ExtAny;
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
    #[must_use]
    fn type_is<T: 'static>(&self) -> bool { self.type_id() == TypeId::of::<T>() }

    /// Returns a deterministic hash of the `TypeId` of `Self`.
    fn type_hash(&self) -> u64 {
        let hasher = HasherFx::<u64>::default();
        self.type_hash_with(hasher)
    }

    /// Returns a deterministic hash of the `TypeId` of `Self` using a custom hasher.
    fn type_hash_with<H: Hasher>(&self, mut hasher: H) -> u64 {
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    /* upcasts */

    /// Upcasts `&self` as `&dyn Any`.
    ///
    /// # Example
    /// ```
    /// use devela::{Any, ExtAny};
    ///
    /// let val = 5;
    /// let any: &dyn Any = &val as &dyn Any;
    /// assert!(any.is::<i32>()); // works direcly for dyn Any
    /// ```
    #[must_use]
    fn as_any_ref(&self) -> &dyn Any where Self: Sized { self }

    /// Upcasts `&mut self` as `&mut dyn Any`.
    ///
    /// # Example
    /// ```
    /// use devela::{Any, ExtAny};
    ///
    /// let mut x = 5;
    /// let any: &mut dyn Any = x.as_any_mut();
    /// assert!(any.is::<i32>());
    /// ```
    #[must_use]
    fn as_any_mut(&mut self) -> &mut dyn Any where Self: Sized { self }

    /// Upcasts `Box<self>` as `Box<dyn Any>`.
    ///
    /// # Example
    /// ```
    /// use devela::{Any, ExtAny};
    ///
    /// let x = Box::new(5);
    /// let any: Box<dyn Any> = x.as_any_box();
    /// assert!(any.is::<i32>());
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn as_any_box(self: Box<Self>) ->  Box<dyn Any> where Self: Sized { self }

    /* downcasts */

    /// Returns some shared reference to the inner value if it is of type `T`.
    ///
    /// This method is only needed when not dealing directly with `dyn Any` trait objects,
    /// since it's [already implemented for `dyn Any`](Any#method.downcast_ref).
    ///
    /// # Example
    /// ```
    /// use core::fmt::Display;
    /// use devela::{Any, ExtAny};
    ///
    /// trait Trait: Any + Display {}
    /// impl Trait for i32 {}
    /// impl Trait for char {}
    /// impl Trait for bool {}
    ///
    /// # #[cfg(feature = "alloc")]
    /// // in the heap:
    /// {
    ///     # use devela::{Box, Vec};
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
    /// # #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))] // FIX
    /// {
    ///     use devela::{Any, DstArray, DstStack, DstValue, ExtAny};
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
    #[must_use]
    #[cfg(feature = "unsafe_layout")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        // SAFETY: We verify T is of the right type before downcasting
        unsafe { (*self).type_is::<T>().then(|| &mut *<*mut _>::cast(self)) }
    }
}

#[cfg(test)]
mod tests {
    use crate::ExtAny;

    #[test]
    fn closure_type_ids() {
        let closure1 = || {};
        let closure2 = || {};
        let closure_with_env = |x: i32| x + 1;

        // Ensure `type_of` produces unique `TypeId`s for each closure.
        assert_ne!(closure1.type_of(), closure2.type_of());
        assert_ne!(closure1.type_of(), closure_with_env.type_of());
        assert_ne!(closure2.type_of(), closure_with_env.type_of());

        // All closure names in the same module are the same.
        assert_eq!(closure1.type_name(), closure2.type_name());
        assert_eq!(closure1.type_name(), closure_with_env.type_name());
    }
}
