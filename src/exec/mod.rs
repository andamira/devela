// devela::exec
//
//! Execution strategies and concurrency management.
#![doc = crate::code::doc_extends!(future, process, sync, task, thread)]
//!
//

// safety:
#![cfg_attr(feature = "safe_exec", forbid(unsafe_code))]

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
