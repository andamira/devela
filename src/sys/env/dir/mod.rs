//
//! Defines [`AppData`], [`DirEnv`], [`DirApple`], [`DirUnix`], [`DirWindows`] and [`DirXdg`].
//

use crate::{iif, Env, Path, PathBuf};

/// Application specific metadata.
///
/// It is used together with [`DirEnv`].
#[derive(Clone, Debug, PartialEq)]
pub struct AppData {
    tld: String,
    author: String,
    app_name: String,
}
impl AppData {
    /// Creates a new `AppData` if all fields are valid.
    ///
    /// In order to be valid, they can't be empty, and:
    /// - <abbr title = "Top Level Domain">`tld`</abbr>:
    ///   up to 127 lowercase alphanumeric characters and dots (`^[a-z0-9\.]+$`).
    /// - `author`:
    ///   up to 50 alphanumeric characters, dashes, and spaces (`^[0-9A-Za-z\s\-]+$`).
    /// - `app_name`:
    ///   up to 50 alphanumeric characters and spaces (`^[0-9A-Za-z\s]+$`).
    pub fn new(tld: &str, author: &str, app_name: &str) -> Option<Self> {
        if Self::validate_tld(tld)
            && Self::validate_author(author)
            && Self::validate_app_name(app_name)
        {
            None
        } else {
            Some(Self {
                tld: tld.into(),
                author: author.into(),
                app_name: app_name.into(),
            })
        }
    }
    /// Gets the *Top Level Domain* of the application (e.g. `com` or `io.github`).
    #[must_use]
    pub fn tld(&self) -> &str {
        &self.tld
    }
    /// Gets the name of the author of the application.
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }
    /// Gets the name of the application.
    #[must_use]
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    fn validate_tld(tld: &str) -> bool {
        !tld.is_empty()
            && tld.len() <= 127
            && tld.split('.').all(|label| {
                !label.is_empty()
                    && label.len() <= 63
                    && label.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            })
    }
    fn validate_author(author: &str) -> bool {
        !author.is_empty()
            && author.len() <= 50
            && author.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace() || c == '-')
    }
    fn validate_app_name(app_name: &str) -> bool {
        !app_name.is_empty()
            && app_name.len() <= 50
            && app_name.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace())
    }

    /// Returns an Apple bundle identifier.
    ///
    /// This is used in [`DirApple`].
    #[must_use]
    pub fn bundle_id(&self) -> String {
        let author = self.author.to_lowercase().replace(' ', "-");
        let app_name = self.app_name.replace(' ', "-");
        let mut parts = vec![self.tld.as_str(), author.as_str(), app_name.as_str()];
        parts.retain(|part| !part.is_empty());
        parts.join(".")
    }

    /// Returns a ‘unixy’ version of the application’s name, akin to what would
    /// usually be used as a binary name.
    ///
    /// Replaces whitespaces with underscores.
    ///
    /// This is used in [`DirUnix`] and [`DirXdg`].
    #[must_use]
    pub fn unixy_name(&self) -> String {
        self.app_name.to_lowercase().replace(' ', "_")
    }
}

/// Manages directory paths in an environment-aware manner.
///
/// # Derived Work
#[doc = include_str!("./MODIFICATIONS.md")]
#[rustfmt::skip]
pub trait DirEnv {
    /// Gets the home directory.
    #[must_use]
    fn dir_home(&self) -> &Path;

    /// Gets the configuration directory.
    #[must_use]
    fn dir_config(&self) -> PathBuf;

    /// Gets the data directory.
    #[must_use]
    fn dir_data(&self) -> PathBuf;

    /// Gets the cache directory.
    #[must_use]
    fn dir_cache(&self) -> PathBuf;

    /// Gets the state directory.
    ///
    /// Currently, only the [`Xdg`](struct.Xdg.html) & [`DirUnix`] environments support this.
    #[must_use]
    fn dir_state(&self) -> Option<PathBuf>;

    /// Gets the runtime directory.
    ///
    /// Currently, only the [`Xdg`](struct.Xdg.html) & [`DirUnix`] environments support this.
    ///
    /// Note: The [XDG Base Directory Specification](spec) places additional
    /// requirements on this directory related to ownership, permissions, and
    /// persistence. This implementation does not check those requirements.
    ///
    /// [spec]: https://specifications.freedesktop.org/basedir-spec/latest/
    #[must_use]
    fn dir_runtime(&self) -> Option<PathBuf>;

    /* provided methods */

    // NOTE: they accept &Path instead of AsRef<OsStr> to be dyn-compatible.
    // Can be called using .as_ref(), from &str, String, OsStr and OsString.

    /// Constructs a path inside your application’s configuration directory.
    #[must_use]
    fn dir_in_config(&self, append: &Path) -> PathBuf {
        dir_in(self.dir_config(), append)
    }

    /// Constructs a path inside your application’s data directory.
    #[must_use]
    fn dir_in_data(&self, append: &Path) -> PathBuf {
        dir_in(self.dir_data(), append)
    }

    /// Constructs a path inside your application’s cache directory.
    #[must_use]
    fn dir_in_cache(&self, append: &Path) -> PathBuf {
        dir_in(self.dir_cache(), append)
    }

    /// Constructs a path inside your application’s state directory.
    ///
    /// Currently, only the [`Xdg`](struct.Xdg.html) & [`DirUnix`] environments support this.
    #[must_use]
    fn dir_in_state(&self, append: &Path) -> Option<PathBuf> {
        self.dir_state().map(|base| dir_in(base, append))
    }

    /// Constructs a path inside your application’s runtime directory.
    ///
    /// Currently, only the [`Xdg`](struct.Xdg.html) & [`DirUnix`] environments support this.
    #[must_use]
    fn dir_in_runtime(&self, append: &Path) -> Option<PathBuf> {
        self.dir_runtime().map(|base| dir_in(base, append))
    }
}

/// Constructs a path by appending the given `path` to the provided `base` path.
#[must_use] #[rustfmt::skip]
fn dir_in(mut base: PathBuf, path: &Path) -> PathBuf { base.push(path); base }

/// Xdg enviroment for directories.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirXdg {
    home: PathBuf,
    unixy_name: String,
}
impl DirXdg {
    /// Creates a new Xdg directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use]
    pub fn new(app_data: Option<AppData>) -> Option<Self> {
        let home = Env::home_dir()?;
        if let Some(app) = app_data {
            Some(Self { home, unixy_name: app.unixy_name() })
        } else {
            Some(Self { home, unixy_name: String::new() })
        }
    }
    // Returns `None` if the path obtained from the env var isn’t absolute.
    fn env_var_or_none(env_var: &str) -> Option<PathBuf> {
        Env::var(env_var).ok().and_then(|path| {
            let path = PathBuf::from(path);
            path.is_absolute().then_some(path)
        })
    }
    fn env_var_or_default(&self, env_var: &str, default: impl AsRef<Path>) -> PathBuf {
        Self::env_var_or_none(env_var).unwrap_or_else(|| self.home.join(default))
    }
}
impl DirEnv for DirXdg {
    fn dir_home(&self) -> &Path {
        &self.home
    }
    fn dir_config(&self) -> PathBuf {
        let dir = self.env_var_or_default("XDG_CONFIG_HOME", ".config/");
        iif![self.unixy_name.is_empty(); dir; dir.join(&self.unixy_name)]
    }
    fn dir_data(&self) -> PathBuf {
        let dir = self.env_var_or_default("XDG_DATA_HOME", ".local/share/");
        iif![self.unixy_name.is_empty(); dir; dir.join(&self.unixy_name)]
    }
    fn dir_cache(&self) -> PathBuf {
        let dir = self.env_var_or_default("XDG_CACHE_HOME", ".cache/");
        iif![self.unixy_name.is_empty(); dir; dir.join(&self.unixy_name)]
    }
    fn dir_state(&self) -> Option<PathBuf> {
        let dir = self.env_var_or_default("XDG_STATE_HOME", ".local/state/");
        Some(iif![self.unixy_name.is_empty(); dir; dir.join(&self.unixy_name)])
    }
    fn dir_runtime(&self) -> Option<PathBuf> {
        let dir = Self::env_var_or_none("XDG_RUNTIME_DIR");
        iif![self.unixy_name.is_empty(); dir; dir.map(|d| d.join(&self.unixy_name))]
    }
}

/// Unix enviroment for directories.
///
/// This constructs directories specific to an application using
/// its `unixy_name`, which is derived from the application's name.
///
/// This is no standard or official specification, but an old convention of
/// placing the configuration directory under the user's home directory.
///
/// Vim and Cargo are notable examples whose configuration/data/cache directory
/// layouts are similar to these.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirUnix {
    home: PathBuf,
    unixy_name: String,
}
impl DirUnix {
    /// Creates a new Unix directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use]
    pub fn new(app_data: AppData) -> Option<Self> {
        let home = Env::home_dir()?;
        Some(Self { home, unixy_name: app_data.unixy_name() })
    }
}
#[rustfmt::skip]
impl DirEnv for DirUnix {
    fn dir_home(&self) -> &Path { &self.home }
    fn dir_config(&self) -> PathBuf { self.home.join(&self.unixy_name) }
    fn dir_data(&self) -> PathBuf { self.dir_config().join("data") }
    fn dir_cache(&self) -> PathBuf { self.dir_config().join("cache") }
    fn dir_state(&self) -> Option<PathBuf> { Some(self.dir_config().join("state")) }
    fn dir_runtime(&self) -> Option<PathBuf> { Some(self.dir_config().join("runtime")) }
}

/// Apple enviroment for directories.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirApple {
    home: PathBuf,
    bundle_id: String,
}
impl DirApple {
    /// Creates a new Apple directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use]
    pub fn new(app_data: Option<AppData>) -> Option<Self> {
        let home = Env::home_dir()?;
        if let Some(app) = app_data {
            Some(Self { home, bundle_id: app.bundle_id() })
        } else {
            Some(Self { home, bundle_id: String::new() })
        }
    }
}
#[rustfmt::skip]
impl DirEnv for DirApple {
    fn dir_home(&self) -> &Path { &self.home }
    fn dir_config(&self) -> PathBuf {
        let dir = self.home.join("Library/Preferences/");
        iif![self.bundle_id.is_empty(); dir; dir.join(&self.bundle_id)]
    }
    fn dir_data(&self) -> PathBuf {
        let dir = self.home.join("Library/Application Support/");
        iif![self.bundle_id.is_empty(); dir; dir.join(&self.bundle_id)]
    }
    fn dir_cache(&self) -> PathBuf {
        let dir = self.home.join("Library/Caches/");
        iif![self.bundle_id.is_empty(); dir; dir.join(&self.bundle_id)]
    }
    fn dir_state(&self) -> Option<PathBuf> { None }
    fn dir_runtime(&self) -> Option<PathBuf> { None }
}

/// Windows enviroment for directories.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirWindows {
    home: PathBuf,
    app_path: Option<PathBuf>,
}
impl DirWindows {
    /// Creates a new Windows directory environment.
    ///
    /// Returns `None` if the home directory cannot be determined (see [`Env::home_dir`]),
    #[must_use] #[rustfmt::skip]
    pub fn new(app_data: Option<AppData>) -> Option<Self> {
        let home = Env::home_dir()?;
        if let Some(app) = app_data {
            Some(Self { home, app_path: Some(PathBuf::from(app.author).join(app.app_name)) })
        } else {
            Some(Self { home, app_path: None })
        }
    }
}
#[rustfmt::skip]
impl DirEnv for DirWindows {
    fn dir_home(&self) -> &Path { &self.home }
    fn dir_config(&self) -> PathBuf {
        let mut dir = self.home.join("AppData").join("Roaming");
        iif![let Some(app) = &self.app_path; {dir.push(app); dir.push("config"); dir }; dir]
    }
    fn dir_data(&self) -> PathBuf {
        let mut dir = self.home.join("AppData").join("Roaming");
        iif![let Some(app) = &self.app_path; {dir.push(app); dir.push("data"); dir }; dir]
    }
    fn dir_cache(&self) -> PathBuf {
        let mut dir = self.home.join("AppData").join("Local");
        iif![let Some(app) = &self.app_path; {dir.push(app); dir.push("cache"); dir }; dir]
    }
    fn dir_state(&self) -> Option<PathBuf> { None }
    fn dir_runtime(&self) -> Option<PathBuf> { None }
}
