// devela/src/work/exec/thread/sleep/macro.rs

crate::CONST! {
    _DOCS_SLEEP4 = r#"
Sleeps for `$s` seconds, plus optional milliseconds, microseconds and nanoseconds.

Forms:
- `sleep4![s]`
- `sleep4![s, ms]`
- `sleep4![s, ms, us]`
- `sleep4![s, ms, us, ns]`

The single-argument form also accepts floating-point literals:
`sleep4![0.5]`.

# Examples
```
# use devela::sleep4;
sleep4![1, 0, 500]; // sleeps for 1 second + 500 microseconds
sleep4![1.5]; // sleeps for 1.5 seconds
```

# Features
- With the `std` feature enabled it leverages [`Thread::sleep`].
- With the `linux` feature enabled it leverages [`Linux::sleep`].

[`Thread::sleep`]: crate::Thread#method.sleep
[`Linux::sleep`]: crate::Linux::sleep
"#;
}

// std version
#[doc = crate::_tags!(concurrency time)]
/// A more compact thread sleep.
#[doc = crate::_doc_meta!{location("work/thread")}]
#[doc = _DOCS_SLEEP4!()]
#[macro_export]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "linux", feature = "std"))))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! sleep4 {
    ($s:literal) => {
        ::std::thread::sleep($crate::Duration::from_secs_f64($s as f64));
    };
    ($s:expr) => {
        ::std::thread::sleep($crate::Duration::from_secs($s));
    };
    ($s:expr, $ms:expr) => {
        ::std::thread::sleep($crate::Duration::from_millis($s * 1000 + $ms));
    };
    ($s:expr, $ms:expr, $us:expr) => {
        ::std::thread::sleep($crate::Duration::from_micros($s * 1_000_000 + $ms * 1_000 + $us));
    };
    ($s:expr, $ms:expr, $us:expr, $ns:expr) => {
        ::std::thread::sleep($crate::Duration::from_nanos(
            $s * 1_000_000_000 + $ms * 1_000_000 + $us * 1_000 + $ns,
        ));
    };
}

// linux version
#[doc = crate::_tags!(concurrency time)]
/// A more compact thread sleep.
#[doc = crate::_doc_meta!{location("work/thread")}]
#[doc = _DOCS_SLEEP4!()]
#[macro_export]
#[crate::macro_apply(crate::_linux_syscall_not_std)]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "linux", feature = "std"))))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! sleep4 {
    ($s:literal) => {
        $crate::Linux::sleep($crate::Duration::from_secs_f64($s as f64))
            .expect("Linux::sleep failed");
    };
    ($s:expr) => {
        $crate::Linux::sleep($crate::Duration::from_secs($s)).expect("Linux::sleep failed");
    };
    ($s:expr, $ms:expr) => {
        $crate::Linux::sleep($crate::Duration::from_millis($s * 1000 + $ms))
            .expect("Linux::sleep failed");
    };
    ($s:expr, $ms:expr, $us:expr) => {
        $crate::Linux::sleep($crate::Duration::from_micros($s * 1_000_000 + $ms * 1_000 + $us))
            .expect("Linux::sleep failed");
    };
    ($s:expr, $ms:expr, $us:expr, $ns:expr) => {
        $crate::Linux::sleep($crate::Duration::from_nanos(
            $s * 1_000_000_000 + $ms * 1_000_000 + $us * 1_000 + $ns,
        ))
        .expect("Linux::sleep failed");
    };
}

#[cfg(any(feature = "std", all(feature = "linux", not(feature = "std"), not(miri))))]
#[doc(inline)]
pub use sleep4;
