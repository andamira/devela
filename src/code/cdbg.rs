// devela::code::cdbg
//
//!
//

/// A *`c`ustomizable [`dbg!`]*
///
/// - It uses `{:?}` instead of `{:#?}` for formatting.
/// - By default it doesn't show the location (file, line and column).
/// - It can also show the filename, or the `$n` last components, instead of the full path.
///
/// # Examples
/// ```
/// use devela::cdbg;
///
/// let a = vec![1, 2, 3];
/// let _ = cdbg![&a];
/// //      ^-- prints: &a = [1, 2, 3]
/// let _ = cdbg![@ &a];
/// //      ^-- prints: [main.rs:6:10] &a = [1, 2, 3]
/// let _ = cdbg![2@ &a];
/// //      ^-- prints: [src/main.rs:8:10] &a = [1, 2, 3]
/// let _ = cdbg![f@ &a];
/// //      ^-- prints: [/full/path/.../src/main.rs:10:9] &a = [1, 2, 3]
/// ```
// Source code is based on Rust std `dbg!` implementation
#[macro_export]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
macro_rules! cdbg {
    (
     // don't show the location
     $v:expr $(,)?) => { match $v { v => { eprintln!("{} = {:?}", stringify!($v), &v); v } } };
    ($($v:expr),+ $(,)?) => { ($($crate::code::cdbg![$v]),+,) };
    () => { () };

    (
     // show the filename location
     @ $v:expr $(,)?) => {{
        let path = std::path::Path::new(file![]);
        let file = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
        eprint!("[{}:{}:{}] ", file, line!(), column!());
        $crate::code::cdbg![$v]
    }};
    (@ $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![@ $v]),+,) };
    (@) => {{
        let path = std::path::Path::new(file![]);
        let file = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
        eprintln!("[{}:{}:{}]", file, line!(), column!())
    }};

    (
     // show the last $n location components
     $n:literal @ $v:expr $(,)?) => {{
        let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
        for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
            new_path.push(c.as_os_str()); }
        eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        $crate::code::cdbg![$v]
    }};
    ($n:literal @ $($v:expr),+ $(,)?) => { ($( $crate::code::cdbg![$n@ $v] ),+,) };
    ($n:literal @) => {{
        let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
        for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
            new_path.push(c.as_os_str()); }
        eprintln!("[{}:{}:{}]", new_path.display(), line!(), column!())
    }};

    (
     // show the full path location
     f@ $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::code::cdbg![$v]
    }};
    (f@ $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![f@ $v]),+,) };
    (f@) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };

    (
     // show the full path location in a separate line
     fln@ $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::code::cdbg![$v]
    }};
    (fln@ $($v:expr),+ $(,)?) => { ($($crate::code::cdbg![fln@ $v]),+,) };
    (fln@) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };
}
pub use cdbg;
