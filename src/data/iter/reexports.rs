// devela::data::iter::reexports
//
//! Reexported items from `core`.
//
// - WAIT: [iter_map_windows](https://github.com/rust-lang/rust/issues/87155)
// - WAIT: [iter_array_chunks](https://github.com/rust-lang/rust/issues/100450)
// - WAIT: [iter_intersperse](https://github.com/rust-lang/rust/issues/79524)
// - WAIT: [step_trait](https://github.com/rust-lang/rust/issues/42168)
// - WAIT: [trusted_len](https://github.com/rust-lang/rust/issues/37572)
// - WAIT: [trusted_step](https://github.com/rust-lang/rust/issues/85731)
// - WAIT: [iter_from_coroutine](https://github.com/rust-lang/rust/issues/43122) TODO nightly

use crate::code::reexport;

/* types */

reexport! { rust: core::iter,
    doc: "An iterator that links two iterators together, in a chain.",
    @Chain as IterChain
}
reexport! { rust: core::iter,
    doc: "An iterator that clones the elements of an underlying iterator.",
    @Cloned as IterCloned
}
reexport! { rust: core::iter,
    doc: "An iterator that copies the elements of an underlying iterator.",
    @Copied as IterCopied
}
reexport! { rust: core::iter,
    doc: "An iterator that repeats endlessly.",
    @Cycle as IterCycle
}
reexport! { rust: core::iter,
    doc: "An iterator that yields nothing.",
    @Empty as IterEmpty
}
reexport! { rust: core::iter,
    doc: "An iterator that yields the current count and the element during iteration.",
    @Enumerate as IterEnumerate
}
reexport! { rust: core::iter,
    doc: "An iterator that filters the elements of `iter` with `predicate`.",
    @Filter as IterFilter
}
reexport! { rust: core::iter,
    doc: "An iterator that uses `f` to both filter and map elements from `iter`.",
    @FilterMap as IterFilterMap
}
reexport! { rust: core::iter,
    doc: "An iterator that maps each element to an iterator, and yields their elements.",
    @FlatMap as IterFlatMap
}
reexport! { rust: core::iter,
    doc: "An iterator that flattens one level of nesting in an iterator of iterables.",
    @Flatten as IterFlatten
}
reexport! { rust: core::iter,
    doc: "An iterator where each iteration calls the provided closure `F: FnMut() -> Option<T>`.",
    @FromFn as IterFromFn
}
reexport! { rust: core::iter,
    doc: "An iterator that yields `None` forever after the underlying iterator yields `None` once.",
    @Fuse as IterFuse
}
reexport! { rust: core::iter,
    doc: "An iterator that calls a function with a reference to each element before yielding it.",
    @Inspect as IterInspect
}
reexport! { rust: core::iter,
    doc: "An iterator that maps the values of iter with `f`.",
    @Map as IterMap
}
reexport! { rust: core::iter,
    doc: "An iterator that only accepts elements while `predicate` returns `Some(_)`.",
    @MapWhile as IterMapWhile
}
reexport! { rust: core::iter,
    doc: "An iterator that yields an element exactly once.",
    @Once as IterOnce
}
reexport! { rust: core::iter,
    doc: "An iterator that yields a single element `A` by calling `F: FnOnce() -> A`.",
    @OnceWith as IterOnceWith
}
reexport! { rust: core::iter,
    doc: "An iterator with a `peek()` that returns an optional reference to the next element.",
    @Peekable as IterPeekable
}
reexport! { rust: core::iter,
    doc: "An iterator that repeats an element endlessly.",
    @Repeat as IterRepeat
}
reexport! { rust: core::iter,
    doc: "An iterator that endlessly repeats `A` by calling `F: FnMut() -> A`.",
    @RepeatWith as IterRepeatWith
}
reexport! { rust: core::iter,
    doc: "A double-ended iterator with the direction inverted.",
    @Rev as IterRev
}
reexport! { rust: core::iter,
    doc: "An iterator to maintain state while iterating another iterator.",
    @Scan as IterScan
}
reexport! { rust: core::iter,
    doc: "An iterator that skips over `n` elements of `iter`.",
    @Skip as IterSkip
}
reexport! { rust: core::iter,
    doc: "An iterator that rejects elements while `predicate` returns `true`.",
    @SkipWhile as IterSkipWhile
}
reexport! { rust: core::iter,
    doc: "An iterator for stepping iterators by a custom amount.",
    @StepBy as IterStepBy
}
reexport! { rust: core::iter,
    doc: "An iterator where each successive item is computed based on the preceding one.",
    @Successors as IterSuccessors
}
reexport! { rust: core::iter,
    doc: "An iterator that only iterates over the first `n` iterations of `iter`.",
    @Take as IterTake
}
reexport! { rust: core::iter,
    doc: "An iterator that only accepts elements while `predicate` returns `true`.",
    @TakeWhile as IterTakeWhile
}
reexport! { rust: core::iter,
    doc: "An iterator that iterates two other iterators simultaneously.",
    @Zip as IterZip
}

/* traits */

reexport! { rust: core::iter,
    doc: "An iterator able to yield elements from both ends.",
    @DoubleEndedIterator as IteratorDoubleEnded
}
reexport! { rust: core::iter,
    doc: "An iterator that knows its exact length.",
    @ExactSizeIterator as IteratorExactSize
}
reexport! { rust: core::iter,
    doc: "Extend a collection with the contents of an iterator.",
    @Extend as IteratorExtend
}
reexport! { rust: core::iter,
    doc: "Conversion from an [`Iterator`].",
    @FromIterator as IteratorFrom
}
reexport! { rust: core::iter,
    doc: "An iterator that always continues to yield `None` when exhausted.",
    @FusedIterator as IteratorFused
}
reexport! { rust: core::iter,
    doc: "Conversion into an [`Iterator`].",
    @IntoIterator as IteratorInto
}
reexport! { rust: core::iter,
    doc: "A trait for dealing with iterators.",
    Iterator
}
reexport! { rust: core::iter,
    doc: "Represents types that can be created by multiplying elements of an iterator.",
    @Product as IteratorProduct
}
reexport! { rust: core::iter,
    doc: "Represents types that can be created by summing up an iterator.",
    @Sum as IteratorSum
}

/* functions */

reexport! { rust: core::iter,
    extra_features: "nightly_coro",
    doc: "Creates an iterator where each iteration calls the provided coroutine.",
    @from_coroutine as iter_from_coroutine
}
reexport! { rust: core::iter,
    doc: "Creates an iterator that yields nothing.",
    @empty as iter_empty
}
reexport! { rust: core::iter,
    doc: "Creates an iterator that calls the given closure `F: FnMut() -> Option<T>`.",
    @from_fn as iter_from_fn
}
reexport! { rust: core::iter,
    doc: "Creates an iterator that yields an element exactly once.",
    @once as iter_once
}
reexport! { rust: core::iter,
    doc: "Creates an iterator that lazily generates a value exactly once.",
    @once_with as iter_once_with
}
reexport! { rust: core::iter,
    doc: "Creates an iterator that endlessly repeats a single element.",
    @repeat as iter_repeat
}
reexport! { rust: core::iter,
    doc: "Creates an iterator that endlessly repeats calling `F: FnMut() -> A`.",
    @repeat_with as iter_repeat_with
}
reexport! { rust: core::iter,
    doc: "Creates an iterator where each successive item is computed based on the preceding one.",
    @successors as iter_successors
}
reexport! { rust: core::iter,
    doc: "Converts the arguments to iterators and zips them.",
    @zip as iter_zip
}
