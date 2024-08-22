// devela::sys::path::project
//
//!
//

#[cfg(feature = "std")]
use std::{
    convert::AsRef,
    env, fs, io,
    path::{Path, PathBuf},
};

/// Returns an absolute [`PathBuf`], relative to the `crate`'s root.
///
/// It determines the root by finding the first `Cargo.toml` file, from the
/// current directory through its ancestors.
///
/// # Errors
/// Returns an error if it can't find any `Cargo.toml` file,
/// or if it encounters an invalid path during the search process.
///
/// # Examples
/// ```
/// use devela::sys::crate_root;
///
/// match crate_root("") {
///     Ok(p) => println!("Current crate root is {:?}", p),
///     Err(e) => println!("Error obtaining crate root {:?}", e)
/// };
/// ```
#[cfg(not(miri))] // unsupported operation: `getcwd` not available when isolation is enabled
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
pub fn crate_root<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let current_path = env::current_dir()?;
    let mut root_path = current_path.clone();

    for p in current_path.as_path().ancestors() {
        let has_cargo = fs::read_dir(p)?.any(|p| p.unwrap().file_name() == *"Cargo.toml");
        if has_cargo {
            return Ok(root_path.join(path.as_ref()));
        } else {
            root_path.pop();
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}

/// Like [`crate_root`] but returns a [`String`].
///
/// In case of an error the returned string will be empty.
#[cfg(not(miri))]
#[must_use]
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
pub fn crate_root_string<P: AsRef<Path>>(path: P) -> String {
    crate_root(Path::new(path.as_ref())).map_or(String::new(), |p| p.to_str().unwrap().to_owned())
}
