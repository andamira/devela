//
//! Defines [`Version`] and [`VersionFull`].
//

use crate::{ConstInit, impl_trait, is};
use ::core::fmt::{Display, Formatter, Result as FmtResult};

#[doc = crate::_tags!(code)]
/// A compact three-part semantic version core.
#[doc = crate::_doc_meta!{
    location("code/source", struct Version),
    test_size_of(Version = 6|48; niche !Option),
}]
/// Stores the numeric `major.minor.patch` part of a semantic version.
///
/// This type intentionally does not store pre-release or build metadata.
/// It is suitable for compact API versions, backend versions, format versions,
/// and crate-like version identifiers where the numeric components are enough.
///
/// For versions below `1.0.0`, compatibility policy is project-defined:
/// `minor` may still mark breaking changes.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// The incompatible change component.
    pub major: u16,
    /// The compatible feature component.
    pub minor: u16,
    /// The compatible fix component.
    pub patch: u16,
}

impl ConstInit for Version {
    const INIT: Self = Self::ZERO;
}
impl_trait! { fmt::Display for Version |self, f| {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
}}

impl Version {
    /// The zero version: `0.0.0`.º
    pub const ZERO: Self = Self::new(0, 0, 0);

    /// The first stable version: `1.0.0`.
    pub const ONE: Self = Self::new(1, 0, 0);

    /// Returns a new version from its three numeric components.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }

    /// Returns a version from an array of components.
    pub const fn from_array(version: [u16; 3]) -> Self {
        let [major, minor, patch] = version;
        Self::new(major, minor, patch)
    }
    /// Returns the version components as `[major, minor, patch]`.
    #[must_use]
    pub const fn to_array(self) -> [u16; 3] {
        [self.major, self.minor, self.patch]
    }

    /// Returns `true` if all components are zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.major == 0 && self.minor == 0 && self.patch == 0
    }

    /// Returns a copy with a different major component.
    pub const fn with_major(self, major: u16) -> Self {
        Self { major, ..self }
    }
    /// Returns a copy with a different minor component.
    pub const fn with_minor(self, minor: u16) -> Self {
        Self { minor, ..self }
    }
    /// Returns a copy with a different patch component.
    pub const fn with_patch(self, patch: u16) -> Self {
        Self { patch, ..self }
    }

    /// Returns the next major version, resetting minor and patch to zero.
    pub const fn next_major(self) -> Self {
        Self::new(self.major.saturating_add(1), 0, 0)
    }
    /// Returns the next minor version, resetting patch to zero.
    pub const fn next_minor(self) -> Self {
        Self::new(self.major, self.minor.saturating_add(1), 0)
    }
    /// Returns the next patch version.
    pub const fn next_patch(self) -> Self {
        Self::new(self.major, self.minor, self.patch.saturating_add(1))
    }
}

#[doc = crate::_tags!(code)]
/// A semantic version with optional borrowed metadata.
#[doc = crate::_doc_meta!{
    location("code/source", struct VersionFull),
    #[cfg(target_pointer_width = "32")]
    test_size_of(VersionFull = 24|192; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(VersionFull = 40|320; niche !Option),
}]
/// Extends [`Version`] with optional pre-release and build metadata.
///
/// Formats as:
/// - `major.minor.patch`
/// - `major.minor.patch-pre`
/// - `major.minor.patch+build`
/// - `major.minor.patch-pre+build`
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionFull<'a> {
    /// The numeric version core.
    pub version: Version,
    /// Optional pre-release metadata, written after `-`.
    pub pre: Option<&'a str>,
    /// Optional build metadata, written after `+`.
    pub build: Option<&'a str>,
}
impl<'a> ConstInit for VersionFull<'a> {
    const INIT: Self = Self::ZERO;
}
impl Display for VersionFull<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.version, f)?;
        is! { let Some(pre) = self.pre, write!(f, "-{pre}")?; }
        is! { let Some(build) = self.build, write!(f, "+{build}")?; }
        Ok(())
    }
}

// Constructors and setters
impl<'a> VersionFull<'a> {
    /// The zero version: `0.0.0`.
    pub const ZERO: Self = Self::from_version(Version::ZERO);

    /// Returns a new version from its three numeric components.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self::from_version(Version::new(major, minor, patch))
    }
    /// Returns a full version with no metadata.
    pub const fn from_version(version: Version) -> Self {
        Self { version, pre: None, build: None }
    }
    /// Returns a full version from all components.
    pub const fn with(version: Version, pre: Option<&'a str>, build: Option<&'a str>) -> Self {
        Self { version, pre, build }
    }
    /// Returns the numeric version core.
    pub const fn version(self) -> Version {
        self.version
    }
    /// Returns a copy with pre-release metadata.
    pub const fn with_pre(mut self, pre: &'a str) -> Self {
        self.pre = Some(pre);
        self
    }
    /// Returns a copy without pre-release metadata.
    pub const fn without_pre(mut self) -> Self {
        self.pre = None;
        self
    }
    /// Returns a copy with build metadata.
    pub const fn with_build(mut self, build: &'a str) -> Self {
        self.build = Some(build);
        self
    }
    /// Returns a copy without build metadata.
    pub const fn without_build(mut self) -> Self {
        self.build = None;
        self
    }
    /// Returns `true` if no metadata is present.
    #[must_use]
    pub const fn is_core(self) -> bool {
        self.pre.is_none() && self.build.is_none()
    }
}

impl From<Version> for VersionFull<'_> {
    fn from(v: Version) -> Self {
        Self::from_version(v)
    }
}
impl From<VersionFull<'_>> for Version {
    fn from(v: VersionFull) -> Self {
        v.version()
    }
}
