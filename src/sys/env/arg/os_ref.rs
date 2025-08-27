// devela::sys::env::arg::os_ref
//
//! Defines [`IterArgsOsRef`] and [`args_os_ref_iter()`].
//
// WAIT: [macos build fails](https://github.com/dtolnay/argv/issues/1)

use crate::OsStr;

/// Returns an iterator over command line arguments.
pub(crate) fn args_os_ref_iter() -> IterArgsOsRef {
    IterArgsOsRef { platform_specific: r#impl::args_os_ref_iter() }
}

#[doc = crate::TAG_ITERATOR!()]
#[doc = crate::_doc_miri_warn!(tag)]
/// Iterator over references of command line arguments.
///
/// See [`Env::args_os_ref()`][crate::Env#method.args_os_ref].
#[doc = crate::doc_!(vendor: "argv")]
#[derive(Debug)]
pub struct IterArgsOsRef {
    platform_specific: r#impl::IterArgsOsRef,
}

impl Iterator for IterArgsOsRef {
    type Item = &'static OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        self.platform_specific.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.platform_specific.size_hint()
    }
}

impl ExactSizeIterator for IterArgsOsRef {
    fn len(&self) -> usize {
        self.platform_specific.len()
    }
}

// NOTE: miri https://github.com/dtolnay/argv/issues/17#issuecomment-3201054937
#[cfg(all(target_os = "linux", not(target_env = "musl"), not(miri)))]
mod r#impl {
    use crate::{CStr, OsStr, c_char, c_int};
    use std::os::unix::ffi::OsStrExt;
    use std::{mem, ptr};

    static mut ARGC: c_int = 0;
    static mut ARGV: *const *const c_char = ptr::null();

    #[cfg(target_os = "linux")]
    #[unsafe(link_section = ".init_array")]
    #[used]
    static CAPTURE: unsafe extern "C" fn(c_int, *const *const c_char) = capture;

    // Disabled for now until we investigate https://github.com/dtolnay/argv/issues/1
    #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
    #[allow(dead_code)]
    unsafe extern "C" fn capture(argc: c_int, argv: *const *const c_char) {
        unsafe {
            ARGC = argc;
            ARGV = argv;
        }
    }

    pub(crate) fn args_os_ref_iter() -> IterArgsOsRef {
        // These are only mutated before main so they are safe to read once main
        // has begun.
        let argc = unsafe { ARGC };
        let argv = unsafe { ARGV };

        // We count on the OS to provide argv for which argv + argc does not
        // overflow.
        let end = unsafe { argv.offset(argc as isize) };

        IterArgsOsRef { next: argv, end }
    }

    #[derive(Debug)]
    pub(crate) struct IterArgsOsRef {
        next: *const *const c_char,
        end: *const *const c_char,
    }

    impl Iterator for IterArgsOsRef {
        type Item = &'static OsStr;

        fn next(&mut self) -> Option<Self::Item> {
            if ptr::eq(self.next, self.end) {
                None
            } else {
                let ptr = unsafe { *self.next };
                let c_str = unsafe { CStr::from_ptr(ptr) };
                self.next = unsafe { self.next.offset(1) };
                Some(OsStr::from_bytes(c_str.to_bytes()))
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.len();
            (len, Some(len))
        }
    }

    impl ExactSizeIterator for IterArgsOsRef {
        fn len(&self) -> usize {
            (self.end as usize - self.next as usize) / mem::size_of::<*const c_char>()
        }
    }

    // Thread safe despite the raw pointers.
    unsafe impl Send for IterArgsOsRef {}
    unsafe impl Sync for IterArgsOsRef {}
}

#[cfg(any(not(target_os = "linux"), target_env = "musl", miri))]
mod r#impl {
    use crate::{Once, OsStr};
    use std::{env, iter, ptr, slice};

    static ONCE: Once = Once::new();
    static mut ARGV: Vec<&'static OsStr> = Vec::new();

    pub(crate) fn args_os_ref_iter() -> IterArgsOsRef {
        ONCE.call_once(|| {
            let argv = env::args_os()
                .map(|arg| -> &OsStr { Box::leak(arg.into_boxed_os_str()) })
                .collect();
            unsafe { ARGV = argv }
        });
        let argv = unsafe { &*ptr::addr_of!(ARGV) };
        argv.iter().copied()
    }

    pub(crate) type IterArgsOsRef = iter::Copied<slice::Iter<'static, &'static OsStr>>;
}

const _AUTO_TRAITS: () = {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    let _ = assert_send::<IterArgsOsRef>;
    let _ = assert_sync::<IterArgsOsRef>;
};
