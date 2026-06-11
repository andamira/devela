// devela::data::access::iter::namespace
//
//! Defines the [`Iter`] namespace.
//
// WAIT: [iter_chain](https://github.com/rust-lang/rust/issues/125964)

// #[cfg(nightly_coro)]
// use core::iter::from_coroutine;
use core::iter::{empty, from_fn, once, once_with, repeat, repeat_n, repeat_with, successors, zip};

#[doc = crate::_tags!(iterator namespace)]
/// Iterator-related namespaced operations.
#[doc = crate::_doc_meta!{location("data/access/iter")}]
#[derive(Debug)]
pub struct Iter;

/// # Core methods.
impl Iter {
    /// Creates an iterator that yields nothing.
    ///
    /// See `core::iter::`[`empty`].
    pub const fn empty<T>() -> ::core::iter::Empty<T> {
        empty()
    }

    // WAIT: https://github.com/rust-lang/rust/pull/135687
    // /// Creates a new iterator where each iteration calls the provided coroutine.
    // ///
    // /// See `core::iter::`[`from_corooutine`].
    // #[cfg(nightly_coro)]
    // #[cfg_attr(nightly_doc, doc(cfg(nightly_coro)))]
    // pub fn from_coroutine<G>(coroutine: G) -> core::iter::FromCoroutine<G>
    // where
    //     G: crate::Coroutine<Return = ()> + Unpin,
    // {
    //     from_coroutine
    // }

    /// Creates an iterator that calls the given closure `F: FnMut() -> Option<T>`.
    ///
    /// See `core::iter::`[`from_fn`].
    pub fn from_fn<T, F>(f: F) -> ::core::iter::FromFn<F>
    where
        F: FnMut() -> Option<T>,
    {
        from_fn(f)
    }

    /// Creates an iterator that yields an element exactly once.
    ///
    /// See `core::iter::`[`once`].
    pub fn once<T>(value: T) -> ::core::iter::Once<T> {
        once(value)
    }

    /// Creates an iterator that lazily generates a value exactly once.
    ///
    /// See `core::iter::`[`once_with`].
    pub fn once_with<A, F>(make: F) -> ::core::iter::OnceWith<F>
    where
        F: FnOnce() -> A,
    {
        once_with(make)
    }

    /// Creates an iterator that endlessly repeats a single element.
    ///
    /// See `core::iter::`[`repeat`].
    pub fn repeat<T>(elt: T) -> ::core::iter::Repeat<T>
    where
        T: Clone,
    {
        repeat(elt)
    }

    /// Creates a new iterator that repeats a single element a given number of times.
    ///
    /// See `core::iter::`[`repeat_n`].
    pub fn repeat_n<T>(element: T, count: usize) -> ::core::iter::RepeatN<T>
    where
        T: Clone,
    {
        repeat_n(element, count)
    }

    /// Creates an iterator that endlessly repeats calling `F: FnMut() -> A`.
    ///
    /// See `core::iter::`[`repeat_with`].
    pub fn repeat_with<A, F>(repeater: F) -> ::core::iter::RepeatWith<F>
    where
        F: FnMut() -> A,
    {
        repeat_with(repeater)
    }

    /// Creates an iterator where each successive item is computed based on the previous.
    ///
    /// See `core::iter::`[`successors`].
    pub fn successors<T, F>(first: Option<T>, succ: F) -> ::core::iter::Successors<T, F>
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
    ) -> ::core::iter::Zip<<A as IntoIterator>::IntoIter, <B as IntoIterator>::IntoIter>
    where
        A: IntoIterator,
        B: IntoIterator,
    {
        zip(a, b)
    }
}
