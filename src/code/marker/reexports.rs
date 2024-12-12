// devela::code::marker::reexports
//
//! Reexported items from `core::marker`.
//

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
//
// See also
// - <https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns>
pub use core::marker::PhantomData;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::PhantomPinned;

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Copy;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Send;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Sized;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Sync;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Unpin;
