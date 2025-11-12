// devela::code::const_init
//
//! Defines the [`ConstInit`] trait and implements it for many types.
// NOTE: many implementations are scattered around the codebase.
//
// TOC
// - definitions:
//   - trait ConstInit
// - implementations:
//   - mod impl_core
//   - mod impl_std
//   - mod impl_devela

use crate::ConstInitCore;

/* definitions */

/// Provides a const-friendly initializer for types that may use higher-level abstractions.
///
/// Implemented by general types that can freely depend on higher-level traits.
///
/// It is automatically implemented for all **sealed** types that implement [`ConstInitCore`].
///
/// # Comparison with `Default`
///
/// `Default` represents the type’s natural baseline, usually a neutral or zero-like state.
/// `ConstInit` and `ConstInitCore` simply provide a valid const-time initializer,
/// without requiring that meaning.
///
/// When a type has a clear `Default`, these traits should return the same value to keep behavior
/// consistent. They only diverge for types that cannot offer a safe or meaningful `Default`, but
/// still need a guaranteed const initializer.
///
/// - Use `Default` for semantic baselines.
/// - Use `ConstInit` or `ConstInitCore` for invariant-safe const initialization.
pub trait ConstInit {
    /// Returns the compile-time "initial value" for a type.
    const INIT: Self;
}

/// Marker trait to allow parameterized blanked implementation
pub trait Sealed {}
impl<T: ConstInitCore + Sealed> ConstInit for T {
    const INIT: Self = <T as ConstInitCore>::INIT;
}

/* implementations */

#[rustfmt::skip]
mod impl_core {
    use super::{ConstInitCore, Sealed};
    use crate::{
        _impl_init,

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

    _impl_init![%Sealed%ConstInitCore: tuples <T: ConstInitCore>];
    _impl_init![%Sealed%ConstInitCore: arrays <T: ConstInitCore>
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];

    _impl_init!(%Sealed%:
        (), bool, char,
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize,
        f32, f64,
        Duration,
        RangeFull,
        PhantomPinned,
        &CStr, &str, &mut str,
    );
    _impl_init![%Sealed%: <T> *const T, *mut T, &[T]];
    _impl_init![%Sealed%: <T: ConstInitCore>
        RangeFrom<T>, RangeTo<T>, RangeToInclusive<T>, Range<T>, RangeInclusive<T>,
        OnceCell<T>, Cell<T>, ManuallyDrop<T>, RefCell<T>, UnsafeCell<T>,
        PanicAssertUnwindSafe<T>, Reverse<T>, Saturating<T>, Wrapping<T>,
        PhantomData<T>,
        Option<T>,
    ];
    impl<T: ConstInitCore, E> Sealed for Result<T, E> {}
}

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod impl_alloc {
    use crate::{_impl_init, String};

    #[cfg(feature = "alloc")]
    _impl_init![ConstInit: Self::new() => String];

    // TODO: fxhash, fnv, ahash
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![ConstInit: <K, V> Self::with_hasher(TODO) => HashMap<K, V>];
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![ConstInit: <K> Self::with_hasher(TODO) => HashSet<K>];
}

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::_impl_init;
    use std::{
        cell::LazyCell,
        // collections::hash_map::DefaultHasher
        // io::{Cursor, Empty, Sink},
        process::ExitCode,
        sync::{Condvar, LazyLock, Mutex, Once, OnceLock, RwLock, Weak as ArcWeak},
    };
    // Types that don't implement Default:
    // - OsString: OsString { inner: Buf::from_string(String::new()) }

    _impl_init![ConstInit: Self::SUCCESS => ExitCode];
    _impl_init![ConstInit: Self::new() => Condvar, Once];
    _impl_init![ConstInit: <T: ConstInit>
        Self::new() => ArcWeak<T>, OnceLock<T>];
    _impl_init![ConstInit: <T: ConstInit>
        Self::new(T::INIT) => Mutex<T>, RwLock<T>];
    _impl_init![ConstInit: <T: ConstInit>
        Self::new(|| T::INIT) => LazyCell<T>, LazyLock<T>];

    // WAIT: [const_hash](https://github.com/rust-lang/rust/issues/104061)
    // #[cfg(feature = "dep_hashbrown")]
    // _impl_init![ConstInit: <K, V> Self::with_hasher(DefaulHashher) => BTreeMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // _impl_init![ConstInit: Self => Cursor, Empty, Sink];
}

#[rustfmt::skip]
mod impl_devela {
    use crate::{ConstInit, paste, sf};
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

    impl<T: ConstInit> ConstInit for Cast<T> {
        const INIT: Self = Cast(T::INIT);
    }

    /* num */

    /// Provides a *const* default value for `Interval`, the unbounded interval $(-\infty, \infty)$.
    ///
    /// See the [`default`][Self::default] implementation for more information.
    impl<T> ConstInit for Interval<T> { const INIT: Self = Self::unbounded(); }

    impl ConstInit for Sign { #[doc = "No sign."] const INIT: Self = Sign::None; }

    // NOTE: NonExtreme* types have their implementation in num/niche/impls.rs

    /* text */

    sf! {
        impl ConstInit for CharAscii { const INIT: Self = CharAscii::Null; }
        impl ConstInit for char7 { const INIT: Self = char7::MIN; }
        impl ConstInit for char8 { const INIT: Self = char8::MIN; }
        impl ConstInit for char16 { const INIT: Self = char16::MIN; }
        impl ConstInit for charu { const INIT: Self = charu::MIN; }
        impl ConstInit for charu_niche { const INIT: Self = charu_niche::MIN; }
    }

    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> ConstInit for GraphemeNonul<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const INIT: Self = Self::new();
    }
    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> ConstInit for GraphemeU8<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const INIT: Self = Self::new();
    }
    #[cfg(all(feature = "grapheme", feature = "alloc"))]
    impl ConstInit for GraphemeString {
        const INIT: Self = Self::new();
    }

    impl<const CAP: usize> ConstInit for StringNonul<CAP> {
        #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`u8::MAX`]."]
        const INIT: Self = Self::new();
    }
    macro_rules! _impl_init_for_string_u { // impl ConstInit for StringU*
        () => { _impl_init_for_string_u![u8, u16, u32, usize]; };
        ($($t:ty),+ $(,)?) => {
            $( paste! { _impl_init_for_string_u![@[<String $t:camel>], $t]; } )+
        };
        (@$name:ty, $t:ty) => { paste! {
            impl<const CAP: usize> ConstInit for $name<CAP> {
                #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`" $t "::MAX`]."]
                const INIT: Self = Self::new();
            }
        }};
    }
    _impl_init_for_string_u!();
}
