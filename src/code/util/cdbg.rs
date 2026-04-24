// devela::code::util::cdbg
//
//! Defines the custom debug macro [`cdbg!`].
//

#[doc = crate::_tags!(code debug)]
/// *`c`ustomizable [`dbg!`]* macro.
#[doc = crate::_doc_location!("code/util")]
///
/// - By default uses `{:?}` instead of `{:#?}` for formatting.
/// - By default doesn't show the location (file, line and column).
/// - It can show the `$n` last components, instead of the full path.
/// - It can use a string prefix before `;`, replacing the default `expr = ` label.
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
/// // use `;` to indicate a custom prefix:
/// let _ = cdbg!["a: "; &a];
/// //      ^-- prints: a: [1, 2, 3]
/// let _ = cdbg![; &a];
/// //      ^-- prints: [1, 2, 3]
/// let _ = cdbg!["a: "; 1@ &a];
/// //      ^-- prints: [main.rs:16:10] a: [1, 2, 3]
///
/// // use `#` for pretty-printing:
/// let _ = cdbg![# &a];
/// let _ = cdbg![0# &a];    // same as cdbg![# &a]
/// let _ = cdbg![1# &a];
/// let _ = cdbg![f # &a];   // same as `dbg!` macro (notice the required whitespace)
/// let _ = cdbg![fln # &a]; // same as before, but separates the path in a new line
/// ```
// Source code began with Rust std `dbg!` implementation. Look where we are now.
#[macro_export]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cdbg {
    (
     // shows no location (pretty-print)                            cdbg![# x];
     # $v:expr $(,)?) => { $crate::cdbg![%fmt # concat!(stringify!($v), " = "), $v] };
    (# $($v:expr),+ $(,)?) => { ($($crate::cdbg![# $v]),+,) };
    (
     // shows no location                                           cdbg![x];
     //                                                             cdbg![@ x];
     $(@)? $v:expr $(,)?) => { $crate::cdbg![%fmt ? concat!(stringify!($v), " = "), $v] };
    ($($v:expr),+ $(,)?) => { ($($crate::cdbg![$v]),+,) };
    (
     // shows no location with a custom prefix (pretty-print)       cdbg!["x: "; # x];
     $prefix:literal ; # $v:expr $(,)?) => { $crate::cdbg![%fmt # $prefix, $v] };
    ($prefix:literal ; # $($v:expr),+ $(,)?) => { ($($crate::cdbg![$prefix; # $v]),+,) };
    (
     // shows no location with a custom prefix                      cdbg!["x: "; x];
     //                                                             cdbg![""; x];
     $prefix:literal ; $(@)? $v:expr $(,)?) => { $crate::cdbg![%fmt ? $prefix, $v] };
    ($prefix:literal ; $($v:expr),+ $(,)?) => { ($($crate::cdbg![$prefix; $v]),+,) };
    (

     // shows the last $n location components with a custom prefix
     // ($n=0 no location, $n=1 just the filename)                  cdbg!["x: "; 1@ x];
     $prefix:literal ; $n:literal @ $v:expr $(,)?) => {{
        if $crate::cif!(diff($n, 0)) {
            let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
            for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
                new_path.push(c.as_os_str()); }
            eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        }
        $crate::cdbg![$prefix; $v]
    }};
    ($prefix:literal ; $n:literal @ $($v:expr),+ $(,)?) => {
        ($( $crate::cdbg![$prefix; $n@ $v] ),+,)
    };
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
    (// shows the last $n location components with a custom prefix
     // (pretty-print)                                              cdbg!["x: "; 1# x];
     $prefix:literal ; $n:literal # $v:expr $(,)?) => {{
        if $crate::cif!(diff($n, 0)) {
            let (path, mut new_path) = (std::path::Path::new(file!()), std::path::PathBuf::new());
            for c in path.components().rev().take($n).collect::<Vec<_>>().into_iter().rev() {
                new_path.push(c.as_os_str()); }
            eprint!("[{}:{}:{}] ", new_path.display(), line!(), column!());
        }
        $crate::cdbg![$prefix; # $v]
    }};
    ($prefix:literal ; $n:literal # $($v:expr),+ $(,)?) => {
        ($( $crate::cdbg![$prefix; $n # $v] ),+,)
    };
    (// shows the last $n location components                       cdbg![1# x];
     // (pretty-print)
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

     // shows the full path location with a custom prefix           cdbg!["x: "; f@ x];
     $prefix:literal ; f @ $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::cdbg![$prefix; $v]
    }};
    ($prefix:literal ; f @ $($v:expr),+ $(,)?) => {
        ($($crate::cdbg![$prefix; f @ $v]),+,)
    };
    (
     // shows the full path location                                cdbg![f@ x];
     f @ $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::cdbg![$v]
    }};
    (f @ $($v:expr),+ $(,)?) => { ($($crate::cdbg![f @ $v]),+,) };
    (f @) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };

    (// shows the full path location with a custom prefix           cdbg!["x: "; f # x];
     // (pretty-print)
     $prefix:literal ; f # $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::cdbg![$prefix; # $v]
    }};
    ($prefix:literal ; f # $($v:expr),+ $(,)?) => {
        ($($crate::cdbg![$prefix; f # $v]),+,)
    };
    (// shows the full path location                                cdbg![f # x];
     // (pretty-print, equivalent to `dbg!`)
     f # $v:expr $(,)?) => {{
        eprint!("[{}:{}:{}] ", file!(), line!(), column!());
        $crate::cdbg![# $v]
    }};
    (f # $($v:expr),+ $(,)?) => { ($($crate::cdbg![f # $v]),+,) };
    (f #) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };

    (
     // shows the full path location in a separate line with a custom prefix
     //                                                             cdbg!["x: "; fln@ x];
     $prefix:literal ; fln @ $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::cdbg![$prefix; $v]
    }};
    ($prefix:literal ; fln @ $($v:expr),+ $(,)?) => {
        ($($crate::cdbg![$prefix; fln @ $v]),+,)
    };
    (
     // shows the full path location in a separate line             cdbg![fln@ x];
     fln @ $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::cdbg![$v]
    }};
    (fln @ $($v:expr),+ $(,)?) => { ($($crate::cdbg![fln @ $v]),+,) };
    (fln @) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };

    (// shows the full path location in a separate line with a custom prefix
     // (pretty-print)                                              cdbg!["x: "; fln # x];
     $prefix:literal ; fln # $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::cdbg![$prefix; # $v]
    }};
    ($prefix:literal ; fln # $($v:expr),+ $(,)?) => {
        ($($crate::cdbg![$prefix; fln # $v]),+,)
    };
    (// shows the full path location in a separate line             cdbg![fln # x];
     // (pretty-print)
     fln # $v:expr $(,)?) => {{
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
        $crate::cdbg![# $v]
    }};
    (fln # $($v:expr),+ $(,)?) => { ($($crate::cdbg![fln # $v]),+,) };
    (fln #) => { eprintln!("[{}:{}:{}]", file!(), line!(), column!()) };

    /* private */
    //
    // prints a debug value with a custom prefix
    (%fmt ? $prefix:expr, $v:expr $(,)?) => {{
        let prefix: &str = $prefix;
        match $v { v => { eprintln!("{}{:?}", prefix, &v); v } }
    }};
    // pretty-prints a debug value with a custom prefix
    (%fmt # $prefix:expr, $v:expr $(,)?) => {{
        let prefix: &str = $prefix;
        match $v { v => { eprintln!("{}{:#?}", prefix, &v); v } }
    }};
    (
    // empty prefix shorthand                                       cdbg![; x];
    ; $($tt:tt)+) => { $crate::cdbg![""; $($tt)+] };
    (
    // no-op                                                        cdbg![;];
    //                                                              cdbg![];
    ;) => { () };
    () => { () };
}
#[doc(inline)]
pub use cdbg;
