//
//! Defines [`AppPathsUnix`].
//

use crate::{AppPaths, Env, Path, PathBuf};

use super::info::AppInfo;

#[doc = crate::_tags!(unix)]
/// Unix environment for directories.
#[doc = crate::_doc_meta!{
    location("sys/fs/app", struct AppPathsUnix),
}]
/// This constructs directories specific to an application using
/// its `unixy_name`, which is derived from the application's name.
///
/// This is no standard or official specification, but an old convention of
/// placing the configuration directory under the user's home directory.
///
/// Vim and Cargo are notable examples whose configuration/data/cache directory
/// layouts are similar to these.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppPathsUnix {
    home: PathBuf,
    unixy_name: String,
}
impl AppPathsUnix {
    /// Creates a new Unix directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use]
    pub fn new(app_info: AppInfo) -> Option<Self> {
        let home = Env::home_dir()?;
        Some(Self { home, unixy_name: app_info.unixy_name() })
    }
}
#[rustfmt::skip]
impl AppPaths for AppPathsUnix {
    fn app_home(&self) -> &Path { &self.home }
    fn app_config(&self) -> PathBuf { self.home.join(&self.unixy_name) }
    fn app_data(&self) -> PathBuf { self.app_config().join("data") }
    fn app_cache(&self) -> PathBuf { self.app_config().join("cache") }
    fn app_state(&self) -> Option<PathBuf> { Some(self.app_config().join("state")) }
    fn app_runtime(&self) -> Option<PathBuf> { Some(self.app_config().join("runtime")) }
}
