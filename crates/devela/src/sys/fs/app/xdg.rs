//
//! Defines [`AppPathsXdg`].
//

use crate::{AppPaths, Env, Path, PathBuf, is};

use super::info::AppInfo;

#[doc = crate::_tags!(linux)]
/// Xdg environment for directories.
#[doc = crate::_doc_meta!{
    location("sys/fs/app", struct AppPathsXdg),
}]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppPathsXdg {
    home: PathBuf,
    unixy_name: String,
}
impl AppPathsXdg {
    /// Creates a new Xdg directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use]
    pub fn new(app_info: Option<AppInfo>) -> Option<Self> {
        let home = Env::home_dir()?;
        if let Some(app) = app_info {
            Some(Self { home, unixy_name: app.unixy_name() })
        } else {
            Some(Self { home, unixy_name: String::new() })
        }
    }
    // Returns `None` if the path obtained from the env var isn't absolute.
    fn env_var_or_none(env_var: &str) -> Option<PathBuf> {
        Env::var_os(env_var).and_then(|path| {
            let path = PathBuf::from(path);
            path.is_absolute().then_some(path)
        })
    }
    fn env_var_or_default(&self, env_var: &str, default: impl AsRef<Path>) -> PathBuf {
        Self::env_var_or_none(env_var).unwrap_or_else(|| self.home.join(default))
    }
}
impl AppPaths for AppPathsXdg {
    fn app_home(&self) -> &Path {
        &self.home
    }
    fn app_config(&self) -> PathBuf {
        let dir = self.env_var_or_default("XDG_CONFIG_HOME", ".config/");
        is![self.unixy_name.is_empty(), dir, dir.join(&self.unixy_name)]
    }
    fn app_data(&self) -> PathBuf {
        let dir = self.env_var_or_default("XDG_DATA_HOME", ".local/share/");
        is![self.unixy_name.is_empty(), dir, dir.join(&self.unixy_name)]
    }
    fn app_cache(&self) -> PathBuf {
        let dir = self.env_var_or_default("XDG_CACHE_HOME", ".cache/");
        is![self.unixy_name.is_empty(), dir, dir.join(&self.unixy_name)]
    }
    fn app_state(&self) -> Option<PathBuf> {
        let dir = self.env_var_or_default("XDG_STATE_HOME", ".local/state/");
        Some(is![self.unixy_name.is_empty(), dir, dir.join(&self.unixy_name)])
    }
    fn app_runtime(&self) -> Option<PathBuf> {
        let dir = Self::env_var_or_none("XDG_RUNTIME_DIR");
        is![self.unixy_name.is_empty(), dir, dir.map(|d| d.join(&self.unixy_name))]
    }
}
