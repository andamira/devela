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
    use crate::{_impl_init,
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
        Duration,
        Cell, OnceCell, RefCell, UnsafeCell,
        PhantomData, PhantomPinned, ManuallyDrop,
        Reverse, Saturating, Wrapping,
        Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
        PanicAssertUnwindSafe,
        CStr,
        // Exclusive,
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
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
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
    // _impl_init![ConstInit: <K, V> Self::with_hasher(DefaulTHasher) => BTreeMap<K, V>];
    // WAIT: [const_io_structs](https://github.com/rust-lang/rust/issues/78812)
    // _impl_init![ConstInit: Self => Cursor, Empty, Sink];
}

// implements Sealed for ConstInitCore impls
#[rustfmt::skip]
mod impl_devela_base_core {
    use super::{ConstInitCore, Sealed};
    use crate::{_impl_init, paste,
        // code
        // data
        // media
        // num
        Cast, Cmp, Cycle, CycleCount, Interval, Sign,
        // text
        CharAscii, char7, char8, char16, charu, charu_niche,
        StringNonul, StringU8, StringU16, StringU32, StringUsize,
        // ui
        // work
    };
    // text
    #[cfg(feature = "grapheme")]
    pub use crate::{GraphemeNonul, GraphemeU8};

    // num
    _impl_init![%Sealed%: Sign];
    _impl_init![%Sealed%: <T: ConstInitCore> Cast<T>, Cmp<T>, Cycle<T>];
    _impl_init![%Sealed%: <T: ConstInitCore, N: ConstInitCore> CycleCount<T, N>];
    _impl_init![%Sealed%: <T> Interval<T>];

    // text
    _impl_init![%Sealed%: CharAscii, char7, char8, char16, charu, charu_niche];
    impl<const CAP: usize> Sealed for StringNonul<CAP> {}
    macro_rules! _stringu {
        () => { _stringu![u8, u16, u32, usize]; };
        ($($t:ty),+ $(,)?) => { $( paste! { _stringu![@[<String $t:camel>], $t]; } )+ };
        (@$name:ident, $t:ty) => { impl<const CAP: usize> Sealed for $name<CAP> {} };
    }
    _stringu!();

    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> Sealed for GraphemeNonul<CAP> {}
    #[cfg(feature = "grapheme")]
    impl<const CAP: usize> Sealed for GraphemeU8<CAP> {}
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod impl_devela_base_alloc {
    #[cfg(feature = "grapheme")]
    impl super::Sealed for crate::GraphemeString {}
}
