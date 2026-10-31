//
//! Defines [`AppInfo`].
//

use crate::{String, vec_ as vec};

#[doc = crate::_tags!(platform)]
/// Application specific metadata.
#[doc = crate::_doc_meta!{
    location("run/app", struct AppInfo),
}]
/// It is used together with [`AppPaths`].
#[derive(Clone, Debug, PartialEq)]
pub struct AppInfo {
    tld: String,
    author: String,
    app_name: String,
}
impl AppInfo {
    /// Creates a new `AppInfo` if all fields are valid.
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
            Some(Self {
                tld: tld.into(),
                author: author.into(),
                app_name: app_name.into(),
            })
        } else {
            None
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
    /// This is used in [`AppPathsApple`].
    #[must_use]
    pub fn bundle_id(&self) -> String {
        let author = self.author.to_lowercase().replace(' ', "-");
        let app_name = self.app_name.replace(' ', "-");
        let mut parts = vec![self.tld.as_str(), author.as_str(), app_name.as_str()];
        parts.retain(|part| !part.is_empty());
        parts.join(".")
    }

    /// Returns a 'unixy' version of the application's name, akin to what would
    /// usually be used as a binary name.
    ///
    /// Replaces whitespaces with underscores.
    ///
    /// This is used in [`AppPathsUnix`] and [`AppPathsXdg`].
    #[must_use]
    pub fn unixy_name(&self) -> String {
        self.app_name.to_lowercase().replace(' ', "_")
    }
}

#[cfg(test)]
mod _test {
    use super::AppInfo;

    #[test]
    fn app_info_accepts_valid_fields() {
        let info = AppInfo::new("io.github", "Example Author", "My App").unwrap();
        assert_eq!(info.tld(), "io.github");
        assert_eq!(info.author(), "Example Author");
        assert_eq!(info.app_name(), "My App");
        assert_eq!(info.bundle_id(), "io.github.example-author.My-App");
        assert_eq!(info.unixy_name(), "my_app");
    }
    #[test]
    fn app_info_rejects_invalid_fields() {
        assert!(AppInfo::new("", "Author", "App").is_none());
        assert!(AppInfo::new("io..github", "Author", "App").is_none());
        assert!(AppInfo::new("COM", "Author", "App").is_none());
        assert!(AppInfo::new("com", "", "App").is_none());
        assert!(AppInfo::new("com", "Author!", "App").is_none());
        assert!(AppInfo::new("com", "Author", "").is_none());
        assert!(AppInfo::new("com", "Author", "App!").is_none());
        assert!(AppInfo::new("com", "Author", &"A".repeat(51)).is_none());
    }
}
