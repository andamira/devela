//
//! Defines [`AppPathsApple`].
//

use crate::{AppPaths, Env, Path, PathBuf, is};

use super::info::AppInfo;

#[doc = crate::_tags!(apple)]
/// Apple environment for directories.
#[doc = crate::_doc_meta!{
    location("sys/fs/app", struct AppPathsApple),
}]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppPathsApple {
    home: PathBuf,
    bundle_id: String,
}
impl AppPathsApple {
    /// Creates a new Apple directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use]
    pub fn new(app_info: Option<AppInfo>) -> Option<Self> {
        let home = Env::home_dir()?;
        if let Some(app) = app_info {
            Some(Self { home, bundle_id: app.bundle_id() })
        } else {
            Some(Self { home, bundle_id: String::new() })
        }
    }
}
#[rustfmt::skip]
impl AppPaths for AppPathsApple {
    fn app_home(&self) -> &Path { &self.home }
    fn app_config(&self) -> PathBuf {
        let dir = self.home.join("Library/Preferences/");
        is![self.bundle_id.is_empty(), dir, dir.join(&self.bundle_id)]
    }
    fn app_data(&self) -> PathBuf {
        let dir = self.home.join("Library/Application Support/");
        is![self.bundle_id.is_empty(), dir, dir.join(&self.bundle_id)]
    }
    fn app_cache(&self) -> PathBuf {
        let dir = self.home.join("Library/Caches/");
        is![self.bundle_id.is_empty(), dir, dir.join(&self.bundle_id)]
    }
    fn app_state(&self) -> Option<PathBuf> { None }
    fn app_runtime(&self) -> Option<PathBuf> { None }
}
