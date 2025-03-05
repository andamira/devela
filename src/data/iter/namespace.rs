// devela::data::iter::namespace
//
//! Defines the [`Iter`] namespace.
//
// WAIT: [iter_chain](https://github.com/rust-lang/rust/issues/125964)

// #[cfg(nightly_coro)]
// use core::iter::from_coroutine;
use core::iter::{empty, from_fn, once, once_with, repeat, repeat_n, repeat_with, successors, zip};

#[doc = crate::TAG_NAMESPACE!()]
#[doc = crate::TAG_ITERATOR!()]
/// Iterator-related namespaced operations.
pub struct Iter;

/// # Core methods.
impl Iter {
    /// Creates an iterator that yields nothing.
    ///
    /// See `core::iter::`[`empty`].
    pub const fn empty<T>() -> crate::IterEmpty<T> {
        empty()
    }

    // WAIT: https://github.com/rust-lang/rust/pull/135687
    // /// Creates a new iterator where each iteration calls the provided coroutine.
    // ///
    // /// See `core::iter::`[`from_corooutine`].
    // #[cfg(nightly_coro)]
    // #[cfg_attr(feature = "nightly_doc", doc(cfg(nightly_coro)))]
    // pub fn from_coroutine<G>(coroutine: G) -> core::iter::FromCoroutine<G>
    // where
    //     G: crate::Coroutine<Return = ()> + Unpin,
    // {
    //     from_coroutine
    // }

    /// Creates an iterator that calls the given closure `F: FnMut() -> Option<T>`.
    ///
    /// See `core::iter::`[`from_fn`].
    pub fn from_fn<T, F>(f: F) -> crate::IterFromFn<F>
    where
        F: FnMut() -> Option<T>,
    {
        from_fn(f)
    }

    /// Creates an iterator that yields an element exactly once.
    ///
    /// See `core::iter::`[`once`].
    pub fn once<T>(value: T) -> crate::IterOnce<T> {
        once(value)
    }

    /// Creates an iterator that lazily generates a value exactly once.
    ///
    /// See `core::iter::`[`once_with`].
    pub fn once_with<A, F>(make: F) -> crate::IterOnceWith<F>
    where
        F: FnOnce() -> A,
    {
        once_with(make)
    }

    /// Creates an iterator that endlessly repeats a single element.
    ///
    /// See `core::iter::`[`repeat`].
    pub fn repeat<T>(elt: T) -> crate::IterRepeat<T>
    where
        T: Clone,
    {
        repeat(elt)
    }

    /// Creates a new iterator that repeats a single element a given number of times.
    ///
    /// See `core::iter::`[`repeat_n`].
    pub fn repeat_n<T>(element: T, count: usize) -> crate::IterRepeatN<T>
    where
        T: Clone,
    {
        repeat_n(element, count)
    }

    /// Creates an iterator that endlessly repeats calling `F: FnMut() -> A`.
    ///
    /// See `core::iter::`[`repeat_with`].
    pub fn repeat_with<A, F>(repeater: F) -> crate::IterRepeatWith<F>
    where
        F: FnMut() -> A,
    {
        repeat_with(repeater)
    }

    /// Creates an iterator where each successive item is computed based on the previous.
    ///
    /// See `core::iter::`[`successors`].
    pub fn successors<T, F>(first: Option<T>, succ: F) -> crate::IterSuccessors<T, F>
    where
        F: FnMut(&T) -> Option<T>,
    {
        successors(first, succ)
    }

    /// Converts the arguments to iterators and zips them.
    ///
    /// See `core::iter::`[`zip`].
    pub fn zip<A, B>(
        a: A,
        b: B,
    ) -> crate::IterZip<<A as IntoIterator>::IntoIter, <B as IntoIterator>::IntoIter>
    where
        A: IntoIterator,
        B: IntoIterator,
    {
        zip(a, b)
    }
}

/// # `itertool` methods.
///
/// ## Features
/// They depend on enabling the `dep_itertools` feature.
#[cfg(feature = "dep_itertools")]
impl Iter {}
