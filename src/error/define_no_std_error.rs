// devela::error::no_std
//
//! Interfaces for working with Errors.
//

use core::{
    any::TypeId,
    fmt::{Debug, Display},
};

// Marker struct to prevent downstream implementations to override the
// `Error::type_id` method, since that can enable unsound downcasting.
mod private {
    #[derive(Debug)]
    pub struct Sealed;
}

/// <span class='stab portability' title='re-exported from rust&#39;s `std`
/// or recreated if `not(std)`'>`?std`</span>
//
/// A trait representing the basic expectations for error values.
///
/// I.e., values of type `E` in [`Result<T, E>`].
///
/// Errors must describe themselves through the [`Display`] and [`Debug`]
/// traits. Error messages are typically concise lowercase sentences without
/// trailing punctuation:
/// ```
/// let err = "NaN".parse::<u32>().unwrap_err();
/// assert_eq!(err.to_string(), "invalid digit found in string");
/// ```
/// Errors may provide cause information. [`Error::source()`] is generally
/// used when errors cross "abstraction boundaries". If one module must report
/// an error that is caused by an error from a lower-level module, it can allow
/// accessing that error via [`Error::source()`]. This makes it possible for the
/// high-level module to provide its own errors while also revealing some of the
/// implementation for debugging.
// WAIT:1.81 [error_in_core](https://github.com/rust-lang/rust/pull/125951)
pub trait Error: Debug + Display {
    /// The lower-level source of this error, if any.
    ///
    /// See <https://doc.rust-lang.org/std/error/trait.Error.html#method.source>.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /// Gets the `TypeId` of `self`.
    #[doc(hidden)]
    fn type_id(&self, _: private::Sealed) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
}

impl Error for core::str::ParseBoolError {}
impl Error for core::str::Utf8Error {}
impl Error for core::num::ParseIntError {}
impl Error for core::num::TryFromIntError {}
impl Error for core::array::TryFromSliceError {}
impl Error for core::num::ParseFloatError {}
impl Error for core::convert::Infallible {}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{Debug, Display, Error};
    use crate::{mem::Box, text::String};
    use crate::_dep::_alloc::{
        borrow::Cow,
        string::{FromUtf16Error, FromUtf8Error},
    };

    impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
        /// Converts a type of [`Error`] into a box of dyn [`Error`].
        ///
        /// See
        /// <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3CE%3E-for-Box%3Cdyn+Error%3E>.
        fn from(err: E) -> Box<dyn Error + 'a> {
            Box::new(err)
        }
    }
    impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {
        /// Converts a type of [`Error`] + [`Send`] + [`Sync`] into a box of
        /// dyn [`Error`] + [`Send`] + [`Sync`].
        ///
        /// See <https://doc.rust-lang.org/std/error/trait.Error.html#method.from-6>.
        fn from(err: E) -> Box<dyn Error + Send + Sync + 'a> {
            Box::new(err)
        }
    }

    impl From<String> for Box<dyn Error> {
        /// Converts a [`String`] into a box of dyn [`Error`].
        ///
        /// See <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3CString%3E-for-Box%3Cdyn+Error%3E>.
        fn from(str_err: String) -> Box<dyn Error> {
            let err1: Box<dyn Error + Send + Sync> = From::from(str_err);
            let err2: Box<dyn Error> = err1;
            err2
        }
    }
    impl From<String> for Box<dyn Error + Send + Sync> {
        /// Converts a [`String`] into a box of dyn [`Error`] + [`Send`] + [`Sync`].
        ///
        /// See
        /// <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3CString%3E-for-Box%3Cdyn+Error+%2B+Send+%2B+Sync%3E>.
        #[inline]
        fn from(err: String) -> Box<dyn Error + Send + Sync> {
            struct StringError(String);

            impl Error for StringError {}

            impl Display for StringError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    Display::fmt(&self.0, f)
                }
            }

            // Purposefully skip printing "StringError(..)"
            impl Debug for StringError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    Debug::fmt(&self.0, f)
                }
            }
            Box::new(StringError(err))
        }
    }

    impl From<&str> for Box<dyn Error> {
        /// Converts a [`str`] into a box of dyn [`Error`].
        ///
        /// [`str`]: prim@str
        ///
        /// See
        /// <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E>.
        fn from(err: &str) -> Box<dyn Error> {
            From::from(String::from(err))
        }
    }
    impl<'a> From<&str> for Box<dyn Error + Send + Sync + 'a> {
        /// Converts a [`str`] into a box of dyn [`Error`] + [`Send`] + [`Sync`].
        ///
        /// [`str`]: prim@str
        ///
        /// See
        /// <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error+%2B+Send+%2B+Sync%3E>.
        #[inline]
        fn from(err: &str) -> Box<dyn Error + Send + Sync + 'a> {
            From::from(String::from(err))
        }
    }

    impl<'a> From<Cow<'a, str>> for Box<dyn Error> {
        /// Converts a [`Cow`] into a box of dyn [`Error`].
        ///
        /// See:
        /// <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3CCow%3C'a,+str%3E%3E-for-Box%3Cdyn+Error%3E>.
        fn from(err: Cow<'a, str>) -> Box<dyn Error> {
            From::from(String::from(err))
        }
    }
    impl<'a, 'b> From<Cow<'b, str>> for Box<dyn Error + Send + Sync + 'a> {
        /// Converts a [`Cow`] into a box of dyn [`Error`] + [`Send`] + [`Sync`].
        ///
        /// See
        /// <https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3CCow%3C'b,+str%3E%3E-for-Box%3Cdyn+Error+%2B+Send+%2B+Sync%3E>.
        fn from(err: Cow<'b, str>) -> Box<dyn Error + Send + Sync + 'a> {
            From::from(String::from(err))
        }
    }

    impl Error for FromUtf8Error {}
    impl Error for FromUtf16Error {}

    impl<T: Error> Error for Box<T> {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Error::source(&**self)
        }
    }

    impl dyn Error {
        /// Attempts to downcast the box to a concrete type.
        #[inline]
        #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
        pub fn downcast<T: Error + 'static>(self: Box<Self>) -> Result<Box<T>, Box<dyn Error>> {
            if self.is::<T>() {
                unsafe {
                    let raw: *mut dyn Error = Box::into_raw(self);
                    Ok(Box::from_raw(raw as *mut T))
                }
            } else {
                Err(self)
            }
        }
    }

    impl dyn Error + Send {
        /// Attempts to downcast the box to a concrete type.
        #[inline]
        #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
        pub fn downcast<T: Error + 'static>(
            self: Box<Self>,
        ) -> Result<Box<T>, Box<dyn Error + Send>> {
            let err: Box<dyn Error> = self;
            <dyn Error>::downcast(err).map_err(|s| unsafe {
                // Reapply the `Send` marker.
                core::mem::transmute::<Box<dyn Error>, Box<dyn Error + Send>>(s)
            })
        }
    }

    impl dyn Error + Send + Sync {
        /// Attempts to downcast the box to a concrete type.
        #[inline]
        #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
        pub fn downcast<T: Error + 'static>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
            let err: Box<dyn Error> = self;
            <dyn Error>::downcast(err).map_err(|s| unsafe {
                // Reapply the `Send + Sync` marker.
                core::mem::transmute::<Box<dyn Error>, Box<dyn Error + Send + Sync>>(s)
            })
        }
    }
}

// Copied from `any.rs`.
impl dyn Error + 'static {
    /// Returns `true` if the boxed type is the same as `T`
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object.
        let boxed = self.type_id(private::Sealed);

        // Compare both `TypeId`s on equality.
        t == boxed
    }

    /// Returns some reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[inline]
    #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Error as *const T)) }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[inline]
    #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Error as *mut T)) }
        } else {
            None
        }
    }
}

impl dyn Error + 'static + Send {
    /// Forwards to the method defined on the type `dyn Error`.
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        <dyn Error + 'static>::is::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[inline]
    #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        <dyn Error + 'static>::downcast_ref::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[inline]
    #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        <dyn Error + 'static>::downcast_mut::<T>(self)
    }
}

impl dyn Error + 'static + Send + Sync {
    /// Forwards to the method defined on the type `dyn Error`.
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        <dyn Error + 'static>::is::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[inline]
    #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        <dyn Error + 'static>::downcast_ref::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[inline]
    #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        <dyn Error + 'static>::downcast_mut::<T>(self)
    }
}
