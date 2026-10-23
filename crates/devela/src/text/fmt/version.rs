//
//! Buffered text encoding for [`Version`] and [`VersionFull`].
//

use crate::{Digits, Slice, Str, Version, VersionFull, is, unwrap, whilst, write_at};

impl Version {
    /// The maximum byte length of `major.minor.patch`.
    ///
    /// This is enough for the largest possible `u16` triplet:
    /// `65535.65535.65535`.
    pub const MAX_LEN: usize = 17;

    /// Returns the byte length needed to write this version.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
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
    pub const fn to_str(self, buf: &mut [u8]) -> Result<&str, usize> {
        let len = unwrap![ok? self.write_to(buf)];
        let slice = Slice::range_to(buf, len);
        cfg_select! {
            all(feature = "unsafe_str", not(feature = "safe_text")) => {
                // SAFETY: `write_to` only writes ASCII digits and dots.
                unsafe { Ok(Str::from_utf8_unchecked(slice)) }
            }
            _ => Ok(unwrap![ok_guaranteed_or_ub Str::from_utf8(slice)]),
        }
    }
}

// Length and writing
impl VersionFull<'_> {
    /// Returns the byte length needed to write this version.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(self) -> usize {
        let mut len = self.version.len();
        is! { let Some(pre) = self.pre, len += 1 + pre.len(); } // '-'
        is! { let Some(build) = self.build, len += 1 + build.len(); } // '+'
        len
    }
    /// Writes this version as `major.minor.patch[-pre][+build]`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns the required length if `buf` is too small.
    pub const fn write_to(self, buf: &mut [u8]) -> Result<usize, usize> {
        let needed = self.len();
        is! { buf.len() < needed, return Err(needed) }
        let mut pos = unwrap![ok_guaranteed_or_ub self.version.write_to(buf)];
        if let Some(pre) = self.pre {
            write_at![buf, +=pos, b'-'];
            let bytes = pre.as_bytes();
            whilst! { i in 0..bytes.len(); { write_at![buf, +=pos, bytes[i]]; }}
        }
        if let Some(build) = self.build {
            write_at![buf, +=pos, b'+'];
            let bytes = build.as_bytes();
            whilst! { i in 0..bytes.len(); { write_at![buf, +=pos, bytes[i]]; }}
        }
        Ok(pos)
    }
    /// Writes this version and returns the written string slice.
    ///
    /// # Errors
    /// Returns the required length if `buf` is too small.
    pub const fn to_str(self, buf: &mut [u8]) -> Result<&str, usize> {
        let len = unwrap![ok? self.write_to(buf)];
        let slice = Slice::range_to(buf, len);
        cfg_select! {
            all(feature = "unsafe_str", not(feature = "safe_text")) => {
                // SAFETY: `write_to` writes the numeric core plus caller-provided `str` slices.
                unsafe { Ok(Str::from_utf8_unchecked(slice)) }
            }
            _ => Ok(unwrap![ok_guaranteed_or_ub Str::from_utf8(slice)]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Version, VersionFull};

    #[test]
    fn numeric_version() {
        let version = Version::new(1, 23, 65535);
        let mut buf = [0; Version::MAX_LEN];
        assert_eq!(version.len(), 10);
        assert_eq!(version.write_to(&mut buf), Ok(10));
        assert_eq!(&buf[..10], b"1.23.65535");
        assert_eq!(version.to_str(&mut buf), Ok("1.23.65535"));

        let mut too_short = [0xAA; 9];
        assert_eq!(version.write_to(&mut too_short), Err(10));
        assert_eq!(too_short, [0xAA; 9]);
    }
    #[test]
    fn full_version_with_unicode_metadata() {
        let version = VersionFull::new(2, 3, 4).with_pre("α").with_build("β");
        let mut buf = [0; 32];
        assert_eq!(version.len(), "2.3.4-α+β".len());
        assert_eq!(version.to_str(&mut buf), Ok("2.3.4-α+β"));
        let mut too_short = [0xAA; 8];
        assert_eq!(version.write_to(&mut too_short), Err(version.len()));
        assert_eq!(too_short, [0xAA; 8]);
    }
}
