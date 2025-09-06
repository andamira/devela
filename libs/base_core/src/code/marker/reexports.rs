// devela_base_core::code::marker::reexports
//
//!
//

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
//
// See also
// - <https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns>
pub use ::core::marker::PhantomData;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::marker::PhantomPinned;

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::marker::Copy;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::marker::Send;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::marker::Sized;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::marker::Sync;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::marker::Unpin;

// /// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
// See also:
// - https://dev-doc.rust-lang.org/stable/unstable-book/library-features/fn-ptr-trait.html
// - [implement FnPtr for all fn pointers](https://github.com/rust-lang/rust/pull/108080)
// #[cfg(nightly_fn)]
// pub use ::core::marker::FnPtr;
