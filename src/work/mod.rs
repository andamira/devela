// devela::work
//
//! Work management, concurrency strategies, and execution control.
#![doc = crate::code::doc_extends!(future, process, sync, task, thread)]
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
