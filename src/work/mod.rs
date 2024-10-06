// devela::work
//
//! Work management, concurrency handling.
#![doc = crate::code::doc_!(extends: future, process, sync, task, thread)]
#![doc = crate::code::doc_!(modules: crate; work)]
#![doc = crate::code::doc_!(newline)]
//!
//

// safety:
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]

mod r#async;
mod sync;
mod thread;
#[allow(unused_imports)]
pub use {r#async::all::*, sync::all::*, thread::all::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#async::all::*, sync::all::*, thread::all::*};
}
