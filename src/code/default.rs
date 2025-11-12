// devela::code::default
//
//! Defines the [`ConstDefault`] trait and implements it for many types.
// NOTE: many implementations are scattered around the codebase.
//
// TOC
// - definitions:
//   - trait ConstDefault
//   - macro impl_cdef!
// - implementations:
//   - mod impl_core
//   - mod impl_std
//   - mod impl_devela

use crate::ConstDefaultCore;

/* definitions */

/// Provides a compile-time default value for a type.
///
/// This trait extends [`ConstDefaultCore`] to cover types from the full
/// ecosystem, including those depending on `alloc` or `std`.
///
/// It is automatically implemented for all **sealed** types that implement
/// [`ConstDefaultCore`].
pub trait ConstDefault {
    /// Returns the compile-time “default value” for a type.
    const DEFAULT: Self;
}

/// Marker trait to allow parameterized blanked implementation
pub trait Sealed {}
impl<T: ConstDefaultCore + Sealed> ConstDefault for T {
    const DEFAULT: Self = <T as ConstDefaultCore>::DEFAULT;
}

/* implementations */

#[rustfmt::skip]
mod impl_core {
    use super::{ConstDefaultCore, Sealed};
    use crate::{
        impl_cdef,

        Reverse,
        Cell, OnceCell, RefCell, UnsafeCell,
        CStr,
        PhantomData, PhantomPinned,
        ManuallyDrop,
        Saturating, Wrapping,
        Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        PanicAssertUnwindSafe,
        // Exclusive,
        Duration,
    };

    /* sealed implementations (in sync with devela_base_core::code::default) */

    impl_cdef![%Sealed%ConstDefaultCore: tuples <T: ConstDefaultCore>];
    impl_cdef![%Sealed%ConstDefaultCore: arrays <T: ConstDefaultCore>
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];

    impl_cdef!(%Sealed%:
        (), bool, char,
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize,
        f32, f64,
        Duration,
        RangeFull,
        PhantomPinned,
        &CStr, &str, &mut str,
    );
    impl_cdef![%Sealed%: <T> *const T, *mut T, &[T]];
    impl_cdef![%Sealed%: <T: ConstDefaultCore>
        RangeFrom<T>, RangeTo<T>, RangeToInclusive<T>, Range<T>, RangeInclusive<T>,
        OnceCell<T>, Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>,
        PanicAssertUnwindSafe<T>, Reverse<T>, Saturating<T>, Wrapping<T>,
        PhantomData<T>,
        Option<T>,
    ];
    impl<T: ConstDefaultCore, E> Sealed for Result<T, E> {}
}

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod impl_alloc {
    use crate::{String, impl_cdef};

    #[cfg(feature = "alloc")]
    impl_cdef![ConstDefault: Self::new() => String];

    // TODO: fxhash, fnv, ahash
    // #[cfg(feature = "dep_hashbrown")]
    // impl_cdef![ConstDefault: <K, V> Self::with_hasher(TODO) => HashMap<K, V>];
    // #[cfg(feature = "dep_hashbrown")]
    // impl_cdef![ConstDefault: <K> Self::with_hasher(TODO) => HashSet<K>];
}

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::impl_cdef;
    use std::{
        cell::LazyCell,
        // collections::hash_map::DefaultHasher
        // io::{Cursor, Empty, Sink},
        process::ExitCode,
        sync::{Condvar, LazyLock, Mutex, Once, OnceLock, RwLock, Weak as ArcWeak},
    };
    // Types that don't implement Default:
    // - OsString: OsString { inner: Buf::from_string(String::new()) }

    impl_cdef![ConstDefault: Self::SUCCESS => ExitCode];
    impl_cdef![ConstDefault: Self::new() => Condvar, Once];
    impl_cdef![ConstDefault: <T: ConstDefault>
        Self::new() => ArcWeak<T>, OnceLock<T>];
    impl_cdef![ConstDefault: <T: ConstDefault>
        Self::new(T::DEFAULT) => Mutex<T>, RwLock<T>];
    impl_cdef![ConstDefault: <T: ConstDefault>
        Self::new(|| T::DEFAULT) => LazyCell<T>, LazyLock<T>];

    // WAIT: [const_hash](https://github.com/rust-lang/rust/issues/104061)
    // #[cfg(feature = "dep_hashbrown")]
    // impl_cdef![ConstDefault: <K, V> Self::with_hasher(DefaulHashher) => BTreeMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // impl_cdef![ConstDefault: Self => Cursor, Empty, Sink];
}

#[rustfmt::skip]
mod impl_devela {
    use crate::{ConstDefault, paste, sf};
    use crate::{
        // data //
        Cast,
        // num //
        // TODO: Cycle, CycleCount
        Interval,
        Sign,
        // text //
        CharAscii,
        char7, char8, char16, charu, charu_niche,
        StringNonul, StringU8, StringU16, StringU32, StringUsize,
    };
    #[cfg(feature = "grapheme")]
    pub use crate::{GraphemeNonul, GraphemeU8};
    #[cfg(all(feature = "grapheme", feature = "alloc"))]
    pub use crate::GraphemeString;

    /* data */

    impl<T: ConstDefault> ConstDefault for Cast<T> {
        const DEFAULT: Self = Cast(T::DEFAULT);
    }

    /* num */

    /// Provides a *const* default value for `Interval`, the unbounded interval $(-\infty, \infty)$.
    ///
    /// See the [`default`][Self::default] implementation for more information.
    impl<T> ConstDefault for Interval<T> { const DEFAULT: Self = Self::unbounded(); }

    impl ConstDefault for Sign { #[doc = "No sign."] const DEFAULT: Self = Sign::None; }

    // NOTE: NonExtreme* types have their implementation in num/niche/impls.rs

    /* text */

    sf! {
        impl ConstDefault for CharAscii { const DEFAULT: Self = CharAscii::Null; }
        impl ConstDefault for char7 { const DEFAULT: Self = char7::MIN; }
        impl ConstDefault for char8 { const DEFAULT: Self = char8::MIN; }
        impl ConstDefault for char16 { const DEFAULT: Self = char16::MIN; }
        impl ConstDefault for charu { const DEFAULT: Self = charu::MIN; }
        impl ConstDefault for charu_niche { const DEFAULT: Self = charu_niche::MIN; }
    }

    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> ConstDefault for GraphemeNonul<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const DEFAULT: Self = Self::new();
    }
    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> ConstDefault for GraphemeU8<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const DEFAULT: Self = Self::new();
    }
    #[cfg(all(feature = "grapheme", feature = "alloc"))]
    impl ConstDefault for GraphemeString {
        const DEFAULT: Self = Self::new();
    }

    impl<const CAP: usize> ConstDefault for StringNonul<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const DEFAULT: Self = Self::new();
    }
    macro_rules! impl_cdef_for_string_u { // impl ConstDefault for StringU*
        () => { impl_cdef_for_string_u![u8, u16, u32, usize]; };
        ($($t:ty),+ $(,)?) => {
            $( paste! { impl_cdef_for_string_u![@[<String $t:camel>], $t]; } )+
        };
        (@$name:ty, $t:ty) => { paste! {
            impl<const CAP: usize> ConstDefault for $name<CAP> {
                #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`" $t "::MAX`]."]
                const DEFAULT: Self = Self::new();
            }
        }};
    }
    impl_cdef_for_string_u!();
}
