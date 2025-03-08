// devela::code::util::cdbg
//
//! Custom debug macro.
//

/// *`c`ustomizable [`dbg!`]* macro.
///
/// - By default uses `{:?}` instead of `{:#?}` for formatting.
/// - By default doesn't show the location (file, line and column).
/// - It can show the `$n` last components, instead of the full path.
///
/// # Examples
/// ```
/// # use devela::cdbg;
/// let a = vec![1, 2, 3];
///
/// let _ = cdbg![&a];
/// let _ = cdbg![@ &a];
/// let _ = cdbg![0@ &a];
/// //      ^-- prints: &a = [1, 2, 3]
/// let _ = cdbg![1@ &a];
/// //      ^-- prints: [main.rs:6:10] &a = [1, 2, 3]
/// let _ = cdbg![2@ &a];
/// //      ^-- prints: [src/main.rs:8:10] &a = [1, 2, 3]
/// let _ = cdbg![f@ &a];
/// //      ^-- prints: [/full/path/.../src/main.rs:10:9] &a = [1, 2, 3]
/// let _ = cdbg![fln@ &a];
/// //      ^-- prints: [/full/path/.../src/main.rs:12:9]
/// //                  &a = [1, 2, 3]
///
/// // use `#` for pretty-printing:
/// let _ = cdbg![# &a];
/// let _ = cdbg![0# &a];    // same as cdbg![# &a]
/// let _ = cdbg![1# &a];
/// let _ = cdbg![f # &a];   // same as `dbg!` macro (notice the required whitespace)
/// let _ = cdbg![fln # &a]; // same as before, but separates the path in a new line
/// ```
// Source code is based on Rust std `dbg!` implementation
#[macro_export]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cdbg {
    // has to be the first arm
    (
     // shows no location (pretty-print)                            cdbg![# x];
     # $v:expr $(,)?) => { match $v { v => { eprintln!("{} = {:#?}", stringify!($v), &v); v } } };
    (# $($v:expr),+ $(,)?) => { ($($crate::cdbg![# $v]),+,) };
    (
     // shows no location                                           cdbg![x];
     //                                                             cdbg![@ x];
     $(@)? $v:expr $(,)?) => { match $v { v => { eprintln!("{} = {:?}", stringify!($v), &v); v } } };
    ($($v:expr),+ $(,)?) => { ($($crate::cdbg![$v]),+,) };
    (

     // shows the last $n location components                       cdbg![1@ x];
     // ($n=0 no location, $n=1 just the filename)
     $n:literal @ $v:expr $(,)?) => {{
        if $crate::cif!(diff($n, 0)) {
            let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
            for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
                new_path.push(c.as_os_str()); }
            eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        }
        $crate::cdbg![$v]
    }};
    ($n:literal @ $($v:expr),+ $(,)?) => { ($( $crate::cdbg![$n@ $v] ),+,) };
    ($n:literal @) => {{
        let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
        for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
            new_path.push(c.as_os_str()); }
        eprintln!("[{}:{}:{}]", new_path.display(), line!(), column!())
    }};
    (// (pretty-print)                                              cdbg![1# x];
     $n:literal # $v:expr $(,)?) => {{
        if $crate::cif!(diff($n, 0)) {
            let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
            for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
                new_path.push(c.as_os_str()); }
            eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        }
        $crate::cdbg![# $v]
    }};
    ($n:literal # $($v:expr),+ $(,)?) => { ($( $crate::cdbg![$n # $v] ),+,) };
    ($n:literal #) => {{
        let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
        for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
            new_path.push(c.as_os_str()); }
        eprintln!("[{}:{}:{}]", new_path.display(), line!(), column!())
    }};
    (

     // shows the full path location                                cdbg![f@ x];
     f @ $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::cdbg![$v]
    }};
    (f @ $($v:expr),+ $(,)?) => { ($($crate::cdbg![f @ $v]),+,) };
    (f @) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    (// (pretty-print) (equivalent to `dbg!`)                       cdbg![f # x];
     f # $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::cdbg![# $v]
    }};
    (f # $($v:expr),+ $(,)?) => { ($($crate::cdbg![f # $v]),+,) };
    (f #) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    (
     // shows the full path location in a separate line             cdbg![fln@ x];
     fln @ $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::cdbg![$v]
    }};
    (fln @ $($v:expr),+ $(,)?) => { ($($crate::cdbg![fln @ $v]),+,) };
    (fln @) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    (// (pretty-print)                                              cdbg![fln # x];
     fln # $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::cdbg![# $v]
    }};
    (fln # $($v:expr),+ $(,)?) => { ($($crate::cdbg![fln # $v]),+,) };
    (fln #) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    // no-op:
    () => { () };
}
#[doc(inline)]
pub use cdbg;
