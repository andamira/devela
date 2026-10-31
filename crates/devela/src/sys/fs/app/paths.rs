//
//! Defines [`AppPaths`].
//

use crate::{Env, Path, PathBuf};

#[doc = crate::_tags!(platform)]
/// Manages directory paths in an environment-aware manner.
#[doc = crate::_doc_meta!{
    location("sys/fs/app", trait AppPaths),
    vendor("etcetera"),
}]
#[rustfmt::skip]
pub trait AppPaths {
    /// Gets the home directory.
    #[must_use]
    fn app_home(&self) -> &Path;

    /// Gets the configuration directory.
    #[must_use]
    fn app_config(&self) -> PathBuf;

    /// Gets the data directory.
    #[must_use]
    fn app_data(&self) -> PathBuf;

    /// Gets the cache directory.
    #[must_use]
    fn app_cache(&self) -> PathBuf;

    /// Gets the state directory.
    ///
    /// Currently, only the [`AppPathsXdg`] & [`AppPathsUnix`] environments support this.
    ///
    #[must_use]
    fn app_state(&self) -> Option<PathBuf>;

    /// Gets the runtime directory.
    ///
    /// Currently, only the [`AppPathsXdg`] & [`AppPathsUnix`] environments support this.
    ///
    /// Note: The [XDG Base Directory Specification][spec] places additional
    /// requirements on this directory related to ownership, permissions, and
    /// persistence. This implementation does not check those requirements.
    ///
    /// [spec]: https://specifications.freedesktop.org/basedir-spec/latest/
    #[must_use]
    fn app_runtime(&self) -> Option<PathBuf>;

    /* provided methods */

    // NOTE: they accept &Path instead of AsRef<OsStr> to be dyn-compatible.
    // Can be called using .as_ref(), from &str, String, OsStr and OsString.

    /// Constructs a path inside your application's configuration directory.
    #[must_use]
    fn app_in_config(&self, append: &Path) -> PathBuf {
        path_in(self.app_config(), append)
    }

    /// Constructs a path inside your application's data directory.
    #[must_use]
    fn app_in_data(&self, append: &Path) -> PathBuf {
        path_in(self.app_data(), append)
    }

    /// Constructs a path inside your application's cache directory.
    #[must_use]
    fn app_in_cache(&self, append: &Path) -> PathBuf {
        path_in(self.app_cache(), append)
    }

    /// Constructs a path inside your application's state directory.
    ///
    /// Currently, only the [`AppPathsXdg`] & [`AppPathsUnix`] environments support this.
    #[must_use]
    fn app_in_state(&self, append: &Path) -> Option<PathBuf> {
        self.app_state().map(|base| path_in(base, append))
    }

    /// Constructs a path inside your application's runtime directory.
    ///
    /// Currently, only the [`AppPathsXdg`] & [`AppPathsUnix`] environments support this.
    #[must_use]
    fn app_in_runtime(&self, append: &Path) -> Option<PathBuf> {
        self.app_runtime().map(|base| path_in(base, append))
    }

    /// Gets the temporary directory.
    ///
    /// Uses the system's temporary directory if available. Falls back to
    /// the application cache directory if the temporary directory is unsuitable.
    #[must_use]
    fn app_temp(&self) -> PathBuf {
        let temp_dir = Env::temp_dir();
        if temp_dir.is_absolute() {
            temp_dir
        } else {
            self.app_cache()
        }
    }

    /// Constructs a path inside the temporary directory.
    #[must_use]
    fn app_in_temp(&self, append: &Path) -> PathBuf {
        path_in(self.app_temp(), append)
    }
}

/// Helps construct a path by appending the given `path` to the provided `base` path.
#[must_use] #[rustfmt::skip]
fn path_in(mut base: PathBuf, path: &Path) -> PathBuf { base.push(path); base }
