// devela::code::panic::namespace
//
//! Defines the [`Panic`] namespace.
//
// - https://doc.rust-lang.org/stable/std/panic/index.html

#[cfg(feature = "std")]
use crate::{
    Any, Box, PanicHookInfo, PanicUnwindSafe, ThreadResult,
    _dep::_std::panic::{catch_unwind, panic_any, resume_unwind, set_hook, take_hook},
};

#[doc = crate::TAG_NAMESPACE!()]
/// Panic-related operations.
pub struct Panic;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
impl Panic {
    /// Invokes a closure, capturing the cause of an unwinding panic if one occurs.
    ///
    /// See `std::panic::`[`catch_unwind`].
    pub fn catch<F: FnOnce() -> R + PanicUnwindSafe, R>(f: F) -> ThreadResult<R> {
        catch_unwind(f)
    }

    /// Panic the current thread with the given message as the panic payload.
    ///
    /// See `std::panic::`[`panic_any`].
    pub fn any<M: 'static + Any + Send>(msg: M) -> ! {
        panic_any(msg)
    }

    /// Triggers a panic without invoking the panic hook.
    ///
    /// See `std::panic::`[`resume_unwind`].
    pub fn resume(payload: Box<dyn Any + Send>) -> ! {
        resume_unwind(payload)
    }

    /// Registers a custom panic hook, replacing the previously registered hook.
    pub fn set_hook(hook: Box<dyn Fn(&PanicHookInfo<'_>) + Sync + Send + 'static>) {
        set_hook(hook);
    }

    /// Unregisters the current panic hook, returns it and registers.
    pub fn take_hook() -> Box<dyn Fn(&PanicHookInfo<'_>) + Sync + Send + 'static> {
        take_hook()
    }
}
