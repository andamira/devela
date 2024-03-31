// devela::code::cdbg
//
//!
//

/// A *`c`ustomizable [`dbg!`]*
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
/// let _ = cdbg![f # &a];   // same as `dbg!` macro
/// let _ = cdbg![fln # &a]; // notice the required whitespace
/// ```
// Source code is based on Rust std `dbg!` implementation
#[macro_export]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
macro_rules! cdbg {
    (
     // doesn't show the location (pretty-print)
     # $v:expr $(,)?) => { match $v { v => { eprintln!("{} = {:#?}", stringify!($v), &v); v } } };
    (# $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![# $v]),+,) };
    (
     // doesn't show the location
     $v:expr $(,)?) => { match $v { v => { eprintln!("{} = {:?}", stringify!($v), &v); v } } };
    ($($v:expr),+ $(,)?) => { ($($crate::code::cdbg![$v]),+,) };
    (

     // shows the last $n location components
     // ($n=0 no location, $n=1 just the filename)
     $n:literal @ $v:expr $(,)?) => {{
        if $crate::code::cif!(diff($n, 0)) {
            let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
            for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
                new_path.push(c.as_os_str()); }
            eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        }
        $crate::code::cdbg![$v]
    }};
    ($n:literal @ $($v:expr),+ $(,)?) => { ($( $crate::code::cdbg![$n@ $v] ),+,) };
    ($n:literal @) => {{
        let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
        for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
            new_path.push(c.as_os_str()); }
        eprintln!("[{}:{}:{}]", new_path.display(), line!(), column!())
    }};
    ( // shows the last $n location components (pretty-print)
     $n:literal # $v:expr $(,)?) => {{
        if $crate::code::cif!(diff($n, 0)) {
            let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
            for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
                new_path.push(c.as_os_str()); }
            eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        }
        $crate::code::cdbg![# $v]
    }};
    ($n:literal # $($v:expr),+ $(,)?) => { ($( $crate::code::cdbg![$n # $v] ),+,) };
    ($n:literal #) => {{
        let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
        for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
            new_path.push(c.as_os_str()); }
        eprintln!("[{}:{}:{}]", new_path.display(), line!(), column!())
    }};
    (

     // shows the full path location
     f @ $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::code::cdbg![$v]
    }};
    (f @ $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![f @ $v]),+,) };
    (f @) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    ( // shows the full path location (pretty-print) (same as `dbg!`)
     f # $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::code::cdbg![# $v]
    }};
    (f # $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![f # $v]),+,) };
    (f #) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    (
     // shows the full path location in a separate line
     fln @ $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::code::cdbg![$v]
    }};
    (fln @ $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![fln @ $v]),+,) };
    (fln @) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    ( // shows the full path location in a separate line (pretty-print)
     fln # $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::code::cdbg![# $v]
    }};
    (fln # $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![fln # $v]),+,) };
    (fln #) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
    // no-op:
    () => { () };
}
pub use cdbg;
