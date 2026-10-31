//
//! Defines [`AppPathsWindows`].
//

use crate::{AppPaths, Env, Path, PathBuf, is};

use super::info::AppInfo;

#[doc = crate::_tags!(windows)]
/// Windows environment for directories.
#[doc = crate::_doc_meta!{
    location("sys/fs/app", struct AppPathsWindows),
}]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppPathsWindows {
    home: PathBuf,
    app_path: Option<PathBuf>,
}
impl AppPathsWindows {
    /// Creates a new Windows directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use] #[rustfmt::skip]
    pub fn new(app_info: Option<AppInfo>) -> Option<Self> {
        let home = Env::home_dir()?;
        if let Some(app) = app_info {
            Some(Self { home, app_path: Some(PathBuf::from(app.author()).join(app.app_name())) })
        } else {
            Some(Self { home, app_path: None })
        }
    }
}
#[rustfmt::skip]
impl AppPaths for AppPathsWindows {
    fn app_home(&self) -> &Path { &self.home }
    fn app_config(&self) -> PathBuf {
        let mut dir = self.home.join("AppData").join("Roaming");
        is![let Some(app) = &self.app_path, {dir.push(app); dir.push("config"); dir }, dir]
    }
    fn app_data(&self) -> PathBuf {
        let mut dir = self.home.join("AppData").join("Roaming");
        is![let Some(app) = &self.app_path, {dir.push(app); dir.push("data"); dir }, dir]
    }
    fn app_cache(&self) -> PathBuf {
        let mut dir = self.home.join("AppData").join("Local");
        is![let Some(app) = &self.app_path, {dir.push(app); dir.push("cache"); dir }, dir]
    }
    fn app_state(&self) -> Option<PathBuf> { None }
    fn app_runtime(&self) -> Option<PathBuf> { None }
}
