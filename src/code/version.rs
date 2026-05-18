// devela::code::version
//
//! Defines [`Version`].
//

use crate::{_impl_init, Digits, Slice, Str, StringU8, impl_trait, is, unwrap, write_at};

// MAYBE
// pub struct VersionFull<'a> {
//     pub version: Version,
//     pub pre: Option<&'a str>,
//     pub build: Option<&'a str>,
// }

#[doc = crate::_tags!(code)]
/// A compact three-part semantic version core.
#[doc = crate::_doc_location!("code")]
///
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

_impl_init! { Self::ZERO => Version }
impl_trait! { fmt::Display for Version |self, f| {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
}}

impl Version {
    /// The maximum byte length of `major.minor.patch`.
    ///
    /// This is enough for the largest possible `u16` triplet:
    /// `65535.65535.65535`.
    pub const MAX_LEN: usize = 17;

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

    /// Returns the byte length needed to write this version.
    #[must_use]
    pub const fn len(self) -> usize {
        Digits(self.major).count_digits10() as usize
            + 1
            + Digits(self.minor).count_digits10() as usize
            + 1
            + Digits(self.patch).count_digits10() as usize
    }

    /// Writes this version as `major.minor.patch`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns the required length if `buf` is too small.
    pub const fn write_to(self, buf: &mut [u8]) -> Result<usize, usize> {
        let needed = self.len();
        is! { buf.len() < needed, return Err(needed) }
        let mut pos = 0;
        // Safe after `len` check: the full version is always at least 5 bytes.
        pos += Digits(self.major).write_digits10_fast(buf, pos);
        write_at![buf, +=pos, b'.'];
        pos += Digits(self.minor).write_digits10(buf, pos);
        write_at![buf, +=pos, b'.'];
        pos += Digits(self.patch).write_digits10(buf, pos);
        Ok(pos)
    }

    /// Writes this version as `major.minor.patch`.
    ///
    /// Returns the written string slice.
    ///
    /// # Errors
    /// Returns the required length if `buf` is too small.
    pub const fn to_str<'a>(self, buf: &'a mut [u8]) -> Result<&'a str, usize> {
        let len = unwrap![ok? self.write_to(buf)];
        let slice = Slice::range_to(buf, len);
        cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
            // SAFETY: `write_to` only writes ASCII digits and dots.
            unsafe { Ok(Str::from_utf8_unchecked(slice)) }
        } _ => { Ok(unwrap![ok_guaranteed_or_ub Str::from_utf8(slice)]) }}
    }

    /// Returns this version as an allocated string.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn to_string(self) -> crate::String {
        crate::format!("{self}")
    }

    /// Returns this version as a fixed-capacity string.
    ///
    /// The capacity is enough for the largest possible `u16` triplet:
    /// `65535.65535.65535`.
    #[must_use]
    pub fn to_string_u8(self) -> StringU8<17> {
        let mut buf = [0u8; Self::MAX_LEN];
        let len = unwrap![ok_guaranteed_or_ub self.write_to(&mut buf)];
        StringU8::<17>::_from_array_len_trusted(buf, len as u8)
    }
}
