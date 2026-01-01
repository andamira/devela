// devela_base_core::data::iter::reexports
//
// - WAIT: [iter_map_windows](https://github.com/rust-lang/rust/issues/87155)
// - WAIT: [iter_array_chunks](https://github.com/rust-lang/rust/issues/100450)
// - WAIT: [iter_intersperse](https://github.com/rust-lang/rust/issues/79524)
// - WAIT: [step_trait](https://github.com/rust-lang/rust/issues/42168)
// - WAIT: [trusted_len](https://github.com/rust-lang/rust/issues/37572)
// - WAIT: [trusted_step](https://github.com/rust-lang/rust/issues/85731)

use crate::{_TAG_ITERATOR, _TAG_LIFETIME, _TAG_LOGIC, _TAG_NO, _TAG_NUM, _reexport};

/* core types */

_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that links two iterators together, in a chain.",
    @Chain as IterChain
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that clones the elements of an underlying iterator.",
    @Cloned as IterCloned
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that copies the elements of an underlying iterator.",
    @Copied as IterCopied
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that repeats endlessly.",
    @Cycle as IterCycle
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_NO!(),
    doc: "An iterator that yields nothing.",
    @Empty as IterEmpty
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that yields the current count and the element during iteration.",
    @Enumerate as IterEnumerate
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that filters the elements of `iter` with `predicate`.",
    @Filter as IterFilter
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that uses `f` to both filter and map elements from `iter`.",
    @FilterMap as IterFilterMap
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that maps each element to an iterator, and yields their elements.",
    @FlatMap as IterFlatMap
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that flattens one level of nesting in an iterator of iterables.",
    @Flatten as IterFlatten
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator where each iteration calls a closure `F: FnMut() -> Option<T>`.",
    @FromFn as IterFromFn
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that yields `None` forever after the underlying yields `None` once.",
    @Fuse as IterFuse
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that calls a fn with a reference to each element before yielding it.",
    @Inspect as IterInspect
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that maps the values of iter with `f`.",
    @Map as IterMap
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_LOGIC!(),
    doc: "An iterator that only accepts elements while `predicate` returns `Some(_)`.",
    @MapWhile as IterMapWhile
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that yields an element exactly once.",
    @Once as IterOnce
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that yields a single element `A` by calling `F: FnOnce() -> A`.",
    @OnceWith as IterOnceWith
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_LIFETIME!(),
    doc: "An iterator with a `peek()` that returns an optional ref to the next element.",
    @Peekable as IterPeekable
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that repeats an element endlessly.",
    @Repeat as IterRepeat
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_NUM!(),
    doc: "An iterator that repeats an element an exact number of times.",
    @RepeatN as IterRepeatN
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that endlessly repeats `A` by calling `F: FnMut() -> A`.",
    @RepeatWith as IterRepeatWith
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "A double-ended iterator with the direction inverted.",
    @Rev as IterRev
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator to maintain state while iterating another iterator.",
    @Scan as IterScan
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that skips over `n` elements of `iter`.",
    @Skip as IterSkip
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_LOGIC!(),
    doc: "An iterator that rejects elements while `predicate` returns `true`.",
    @SkipWhile as IterSkipWhile
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator for stepping iterators by a custom amount.",
    @StepBy as IterStepBy
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator where each successive item is computed based on the preceding one.",
    @Successors as IterSuccessors
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that only iterates over the first `n` iterations of `iter`.",
    @Take as IterTake
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_LOGIC!(),
    doc: "An iterator that only accepts elements while `predicate` returns `true`.",
    @TakeWhile as IterTakeWhile
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that iterates two other iterators simultaneously.",
    @Zip as IterZip
}

/* core traits */

_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator able to yield elements from both ends.",
    @DoubleEndedIterator as IteratorDoubleEnded
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that knows its exact length.",
    @ExactSizeIterator as IteratorExactSize
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "Extend a collection with the contents of an iterator.",
    @Extend as IteratorExtend
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "Conversion from an [`Iterator`].",
    @FromIterator as IteratorFrom
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "An iterator that always continues to yield `None` when exhausted.",
    @FusedIterator as IteratorFused
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "Conversion into an [`Iterator`].",
    @IntoIterator as IteratorInto
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!(),
    doc: "A trait for dealing with iterators.",
    Iterator
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_NUM!(),
    doc: "Represents types that can be created by multiplying elements of an iterator.",
    @Product as IteratorProduct
}
_reexport! { rust: core::iter,
    tag: _TAG_ITERATOR!() _TAG_NUM!(),
    doc: "Represents types that can be created by summing up an iterator.",
    @Sum as IteratorSum
}

/* core functions */

// These are re-exported as methods of the Iter namespace.

_reexport! { rust: core::iter,
    extra_flags:(nightly_coro),
    doc: "Creates an iterator where each iteration calls the provided coroutine.",
    @from_coroutine as iter_from_coroutine
}
_reexport! { rust: core::iter,
    extra_flags:(nightly_coro),
    tag: _TAG_ITERATOR!(),
    doc: "Creates an iterator where each iteration calls the provided coroutine.",
    @FromCoroutine as IterFromCoroutine
}
