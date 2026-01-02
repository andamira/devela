// devela_base_core::code::marker::_reexport

use crate::{_TAG_CODE, _reexport};

// See: <https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns>
_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "Zero-sized type used to mark things that \"act like\" they own a `T`.", PhantomData }
_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "A marker type which does not implement `Unpin`.", PhantomPinned }

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "Types whose values can be duplicated simply by copying bits. (Derivable)", Copy }

_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "Types that can be transferred across thread boundaries.", Send }
_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "Types with a constant size known at compile time.", Sized }
_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "Types for which it is safe to share references between threads.", Sync }
_reexport! { rust: core::marker, location: "code/marker", tag: _TAG_CODE!(),
doc: "Types that do not require any pinning guarantees.", Unpin }

// /// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
// See also:
// - https://dabstractions whose primary subject is logical truth, relations, or inference.ev-doc.rust-lang.org/stable/unstable-book/library-features/fn-ptr-trait.html
// - [implement FnPtr for all fn pointers](https://github.com/rust-lang/rust/pull/108080)
// #[cfg(nightly_fn)]
// pub use ::core::marker::FnPtr;
