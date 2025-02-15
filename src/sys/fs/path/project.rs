// devela::sys::fs::path::project
//
//!
//

#[cfg(not(miri))]
use crate::{AsRef, Env, IoError, IoErrorKind, IoResult, Path, PathBuf};
use std::fs;

/// Resolves a given path relative to the nearest `Cargo.toml` directory.
///
/// This function searches for the nearest `Cargo.toml` file starting from the
/// current working directory and traversing upwards through its ancestors.
/// Once the `Cargo.toml` is found, the provided `path` is appended to its directory.
///
/// # Errors
/// Returns an error if it can't find any `Cargo.toml` file,
/// or if it encounters an invalid path during the search process.
///
/// # Examples
/// ```
/// # use devela::crate_root;
/// match crate_root("") {
///     Ok(p) => println!("Current crate root is {:?}", p),
///     Err(e) => println!("Error obtaining crate root {:?}", e)
/// };
/// ```
#[cfg(not(miri))] // unsupported operation: `getcwd` not available when isolation is enabled
pub fn crate_root<P: AsRef<Path>>(path: P) -> IoResult<PathBuf> {
    let current_path = Env::current_dir()?;
    let mut root_path = current_path.clone();

    for p in current_path.as_path().ancestors() {
        let has_cargo = fs::read_dir(p)?.any(|p| p.unwrap().file_name() == *"Cargo.toml");
        if has_cargo {
            return Ok(root_path.join(path.as_ref()));
        }
        root_path.pop();
    }
    Err(IoError::new(IoErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
}

/// Like [`crate_root`] but returns a [`String`].
///
/// In case of an error the returned string will be empty.
#[cfg(not(miri))]
#[must_use]
#[inline]
pub fn crate_root_string<P: AsRef<Path>>(path: P) -> String {
    crate_root(Path::new(path.as_ref())).map_or(String::new(), |p| p.to_str().unwrap().to_owned())
}
