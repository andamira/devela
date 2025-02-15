// devela::sys::fs::fs_path
//
//! Defines the `FsPath` wrapper.
//
// TOC
// - unix methods
//   - traverse symbolic links
//   - optionally traverse symbolic links
// - windows methods
// - from/into Path/PathBuf

use crate::PathBuf;

#[doc = crate::TAG_NAMESPACE!()]
/// A more featureful wrapper over [`PathBuf`].
///
/// # Table of contents
/// - [General methods](#general-methods)
/// - [Methods that traverse symbolic links](#methods-that-traverse-symbolic-links)
/// - [Methods that *optionally* traverse symbolic links](#methods-that-optionally-traverse-symbolic-links)
/// - [`unix` methods that traverse symbolic links](#unix-methods-that-traverse-symbolic-links)
/// - [`unix` methods that *optionally* traverse symbolic links](#unix-methods-that-optionally-traverse-symbolic-links)
/// - [`windows` methods](#windows-methods)
///
/// # Symlink traversal
/// Most methods traverse symbolic links along the path,
/// while the `_ts` suffixed variants can do so, optionally.
///
/// ```
/// # use devela::FsPath;
/// # fn main() {
/// # let some_path = FsPath::new("some_path");
/// assert_eq![some_path.exists(), some_path.exists_ts(true)];
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FsPath {
    path: PathBuf,
}

mod methods {
    use super::FsPath;
    use crate::{Cow, IoError, IoErrorKind, IoResult, Path, PathBuf, PathDisplay, SystemTime};
    use ::std::{
        env,
        fs::{self, FileType, Metadata, Permissions},
    };

    /// # General methods.
    #[rustfmt::skip]
    impl FsPath {
        /// Returns a `FsPath` from what could've been a [`Path`].
        pub fn new<P: AsRef<Path>>(path: P) -> Self { Self { path: path.as_ref().to_path_buf() } }

        /// Returns the current working directory.
        ///
        /// Calls `std::env::`[`current_dir`][std::env::current_dir] under the hood.
        pub fn from_current_dir() -> IoResult<Self> { Ok(Self { path: env::current_dir()? }) }

        /// Returns a temporary directory.
        ///
        /// Calls `std::env::`[`temp_dir`][std::env::temp_dir] under the hood.
        pub fn from_temp_dir() -> Self { Self { path: env::temp_dir() } }

        /// Returns an absolute `path` relative to the current Rust project's root.
        ///
        /// If a relative `path` is provided, it will be appended.
        ///
        /// The project's root is determined by the first `Cargo.toml` file
        /// found from the current directory up through all the ancestors.
        pub fn from_rust_project_root<P: AsRef<Path>>(path: P) -> IoResult<Self> {
            let current_path = env::current_dir()?;
            let mut root_path = current_path.clone();

            for p in current_path.as_path().ancestors() {
                let has_cargo = fs::read_dir(p)?
                    .any(|p| p.unwrap().file_name() == *"Cargo.toml");
                if has_cargo {
                    if path.as_ref().is_relative() {
                        root_path.push(path);
                    }
                    return Ok(Self { path: root_path });
                } else {
                    root_path.pop();
                }
            }
            Err(IoError::new(IoErrorKind::NotFound, "Ran out of places to find `Cargo.toml`")) }

        /// Returns the canonical, absolute form of the path with all intermediate
        /// components normalized and symbolic links resolved.
        ///
        /// Calls `std::fs::`[`canonicalize`][std::fs::canonicalize] under the hood.
        pub fn canonicalize(&self) -> IoResult<FsPath> {
            self.path.canonicalize().map(|path| Self { path })
        }

        /// Reads a symbolic link, returning the file that the link points to.
        ///
        /// This function will return an error in the following situations,
        /// but is not limited to just these cases:
        /// - path is not a symbolic link.
        /// - path does not exist.
        ///
        /// Calls `std::fs::`[`read_link`][std::fs::read_link] under the hood.
        pub fn read_link(&self) -> IoResult<FsPath> {
            self.path.read_link().map(|path| Self { path })
        }

        /// Is this path absolute?
        pub fn is_absolute(&self) -> bool { self.path.is_absolute() }

        /// Is this path relative?
        pub fn is_relative(&self) -> bool { self.path.is_relative() }

        /// Returns the path without its final component, if there is one.
        ///
        /// Returns `None` if the path terminates in a root or prefix.
        pub fn parent(&self) -> Option<FsPath> { self.path.parent().map(|o| o.into()) }

        /// Truncates `self` to [`self.parent`].
        ///
        /// Returns `false` and does nothing if [`self.parent`] is `None`.
        /// Otherwise, returns `true`.
        ///
        /// [`self.parent`]: FsPath#method.parent
        pub fn pop(&mut self) -> bool { self.path.pop() }

        /// Extends `self` with `path`.
        ///
        /// If `path` is absolute, it replaces the current path.
        ///
        /// Calls `std::path::`[`PathBuf::push`] under the hood.
        pub fn push<P: AsRef<Path>>(&mut self, path: P) { self.path.push(path); }

        /// Returns an object that implements [`Display`][core::fmt::Display].
        pub fn display(&self) -> PathDisplay<'_> { self.as_path().display() }

        /// Yields a [`&str`] slice if the FsPath is valid unicode.
        pub fn to_str(&self) -> Option<&str> { self.path.to_str() }

        /// Converts a `FsPath` to a `Cow<str>`.
        ///
        /// Any non-Unicode sequences are replaced with [`U+FFFD REPLACEMENT CHARACTER`][0].
        ///
        /// [0]: https://doc.rust-lang.org/1.62.1/std/char/constant.REPLACEMENT_CHARACTER.html
        pub fn to_string_lossy(&self) -> Cow<str> { self.path.to_string_lossy() }

        /// Returns the inner [`PathBuf`].
        pub fn into_inner(self) -> PathBuf { self.into() }

        /// Returns an exclusive reference to the inner `PathBuf`.
        pub fn into_ref_mut(&mut self) -> &mut PathBuf { self.into() }

        /// Coerces the inner [`PathBuf`] to a [`Path`] slice.
        pub fn as_path(&self) -> &Path { self.into() }
    }

    /// # Methods that traverse symbolic links
    #[rustfmt::skip]
    impl FsPath {
        /* methods that coerce to bool */

        /// Does this path exist?
        pub fn exists(&self) -> bool { self.path.exists() }
        /// Is this a file?
        pub fn is_file(&self) -> bool { self.path.is_file() }
        /// Is this a directory?
        pub fn is_dir(&self) -> bool { self.path.is_dir() }
        /// Is this a symbolic link?
        pub fn is_symlink(&self) -> bool { self.metadata().is_ok_and(|m| m.is_symlink()) }

        /* methods that return IoResult */

        /// Returns the metadata.
        pub fn metadata(&self) -> IoResult<Metadata> { self.path.metadata() }
        /// Returns the `FileType`.
        pub fn file_type(&self) -> IoResult<FileType> { Ok(self.metadata()?.file_type()) }
        /// Returns the size of the file, in bytes.
        pub fn len(&self) -> IoResult<u64> { Ok(self.metadata()?.len()) }
        /// Returns true if the file size equals zero.
        pub fn is_empty(&self) -> IoResult<bool> { Ok(self.len()? == 0) }
        /// Returns the time of creation.
        pub fn created(&self) -> IoResult<SystemTime> { self.metadata()?.created() }
        /// Returns the time of access.
        pub fn accessed(&self) -> IoResult<SystemTime> { self.metadata()?.accessed() }
        /// Returns the time of modification.
        pub fn modified(&self) -> IoResult<SystemTime> { self.metadata()?.modified() }

        // permissions

        /// Returns the permissions of the file.
        pub fn permissions(&self) -> IoResult<Permissions> { Ok(self.metadata()?.permissions()) }
        /// Is this a read-only file?
        pub fn is_readonly(&self) -> IoResult<bool> { Ok(self.permissions()?.readonly()) }
        /// Sets the read-only flag, returning the previous read-only state.
        pub fn set_readonly(&mut self, readonly: bool) -> IoResult<bool> {
            let prev = self.is_readonly()?;
            self.permissions()?.set_readonly(readonly);
            Ok(prev)
        }
    }

    /// # Methods that *optionally* traverse symbolic links
    impl FsPath {
        /* methods that coerce to bool */

        /// Does this path exist?
        pub fn exists_ts(&self, traverse: bool) -> bool {
            self.metadata_ts(traverse).is_ok()
        }
        /// Is this a file?
        pub fn is_file_ts(&self, traverse: bool) -> bool {
            self.metadata_ts(traverse).is_ok_and(|m| m.is_file())
        }
        /// Is this a directory?
        pub fn is_dir_ts(&self, traverse: bool) -> bool {
            self.metadata_ts(traverse).is_ok_and(|m| m.is_dir())
        }
        /// Is this a symbolic link?
        pub fn is_symlink_ts(&self, traverse: bool) -> bool {
            self.metadata_ts(traverse).is_ok_and(|m| m.is_symlink())
        }

        /* methods that return IoResult */

        /// Returns the metadata that *optionally* traverses symbolic links.
        pub fn metadata_ts(&self, traverse: bool) -> IoResult<Metadata> {
            if traverse {
                self.path.metadata()
            } else {
                self.path.symlink_metadata()
            }
        }

        /// Returns the `FileType`.
        pub fn file_type_ts(&self, traverse: bool) -> IoResult<FileType> {
            Ok(self.metadata_ts(traverse)?.file_type())
        }

        /// Returns the size of the file, in bytes.
        #[allow(clippy::len_without_is_empty)]
        pub fn len_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.len())
        }

        /// Returns true if the file size equals zero.
        pub fn is_empty_ts(&self, traverse: bool) -> IoResult<bool> {
            Ok(self.len_ts(traverse)? == 0)
        }
        /// Returns the time of creation.
        pub fn created_ts(&self, traverse: bool) -> IoResult<SystemTime> {
            self.metadata_ts(traverse)?.created()
        }
        /// Returns the time of access.
        pub fn accessed_ts(&self, traverse: bool) -> IoResult<SystemTime> {
            self.metadata_ts(traverse)?.accessed()
        }
        /// Returns the time of modification.
        pub fn modified_ts(&self, traverse: bool) -> IoResult<SystemTime> {
            self.metadata_ts(traverse)?.modified()
        }

        // permissions

        /// Returns the permissions of the file.
        pub fn permissions_ts(&self, traverse: bool) -> IoResult<Permissions> {
            Ok(self.metadata_ts(traverse)?.permissions())
        }

        /// Is this a read-only file?
        pub fn is_readonly_ts(&self, traverse: bool) -> IoResult<bool> {
            Ok(self.permissions_ts(traverse)?.readonly())
        }

        /// Sets the read-only flag, returning the previous read-only state.
        pub fn set_readonly_ts(&mut self, readonly: bool, traverse: bool) -> IoResult<bool> {
            let prev = self.is_readonly_ts(traverse)?;
            self.permissions_ts(traverse)?.set_readonly(readonly);
            Ok(prev)
        }
    }
}

// - https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html
// - https://doc.rust-lang.org/std/fs/struct.Metadata.html#impl-MetadataExt-1
//
// - https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html
// - https://doc.rust-lang.org/std/fs/struct.Metadata.html#impl-MetadataExt
#[cfg(unix)]
mod unix {
    use super::FsPath;
    use crate::IoResult;
    use std::os::unix::fs::MetadataExt;

    /// # `unix` methods that traverse symbolic links
    #[rustfmt::skip]
    impl FsPath {
        /// Returns the ID of the device containing the file.
        pub fn dev(&self) -> IoResult<u64> { Ok(self.metadata()?.dev()) }
        /// Returns the inode number.
        pub fn ino(&self) -> IoResult<u64> { Ok(self.metadata()?.ino()) }
        /// Returns the rights applied to this file.
        pub fn mode(&self) -> IoResult<u32> { Ok(self.metadata()?.mode()) }
        /// Returns the number of hard links pointing to this file.
        pub fn nlink(&self) -> IoResult<u64> { Ok(self.metadata()?.nlink()) }
        /// Returns the user ID of the owner of this file.
        pub fn uid(&self) -> IoResult<u32> { Ok(self.metadata()?.uid()) }
        /// Returns the group ID of the owner of this file.
        pub fn gid(&self) -> IoResult<u32> { Ok(self.metadata()?.gid()) }
        /// Returns the device ID of this file (if it is a special one).
        pub fn rdev(&self) -> IoResult<u64> { Ok(self.metadata()?.rdev()) }
        /// Returns the total size of this file in bytes.
        pub fn size(&self) -> IoResult<u64> { Ok(self.metadata()?.size()) }
        /// Returns the last access time of the file, in seconds since Unix Epoch.
        pub fn atime(&self) -> IoResult<i64> { Ok(self.metadata()?.atime()) }
        /// Returns the last access time of the file, in nanoseconds since atime.
        pub fn atime_nsec(&self) -> IoResult<i64> { Ok(self.metadata()?.atime_nsec()) }
        /// Returns the last modification time of the file, in seconds since Unix Epoch.
        pub fn mtime(&self) -> IoResult<i64> { Ok(self.metadata()?.mtime()) }
        /// Returns the last modification time of the file, in nanoseconds since mtime.
        pub fn mtime_nsec(&self) -> IoResult<i64> { Ok(self.metadata()?.mtime_nsec()) }
        /// Returns the last status change time of the file, in seconds since Unix Epoch.
        pub fn ctime(&self) -> IoResult<i64> { Ok(self.metadata()?.ctime()) }
        /// Returns the last status change time of the file, in nanoseconds since ctime.
        pub fn ctime_nsec(&self) -> IoResult<i64> { Ok(self.metadata()?.ctime_nsec()) }
        /// Returns the block size for filesystem I/O.
        pub fn blksize(&self) -> IoResult<u64> { Ok(self.metadata()?.blksize()) }
        /// Returns the number of blocks allocated to the file, in 512-byte units.
        pub fn blocks(&self) -> IoResult<u64> { Ok(self.metadata()?.blocks()) }
    }

    /// # `unix` methods that *optionally* traverse symbolic links
    impl FsPath {
        /// Returns the ID of the device containing the file.
        pub fn dev_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.dev())
        }
        /// Returns the inode number.
        pub fn ino_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.ino())
        }
        /// Returns the rights applied to this file.
        pub fn mode_ts(&self, traverse: bool) -> IoResult<u32> {
            Ok(self.metadata_ts(traverse)?.mode())
        }
        /// Returns the number of hard links pointing to this file.
        pub fn nlink_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.nlink())
        }
        /// Returns the user ID of the owner of this file.
        pub fn uid_ts(&self, traverse: bool) -> IoResult<u32> {
            Ok(self.metadata_ts(traverse)?.uid())
        }
        /// Returns the group ID of the owner of this file.
        pub fn gid_ts(&self, traverse: bool) -> IoResult<u32> {
            Ok(self.metadata_ts(traverse)?.gid())
        }
        /// Returns the device ID of this file (if it is a special one).
        pub fn rdev_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.rdev())
        }
        /// Returns the total size of this file in bytes.
        pub fn size_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.size())
        }
        /// Returns the last access time of the file, in seconds since Unix Epoch.
        pub fn atime_ts(&self, traverse: bool) -> IoResult<i64> {
            Ok(self.metadata_ts(traverse)?.atime())
        }
        /// Returns the last access time of the file, in nanoseconds since atime.
        pub fn atime_nsec_ts(&self, traverse: bool) -> IoResult<i64> {
            Ok(self.metadata_ts(traverse)?.atime_nsec())
        }
        /// Returns the last modification time of the file, in seconds since Unix Epoch.
        pub fn mtime_ts(&self, traverse: bool) -> IoResult<i64> {
            Ok(self.metadata_ts(traverse)?.mtime())
        }
        /// Returns the last modification time of the file, in nanoseconds since mtime.
        pub fn mtime_nsec_ts(&self, traverse: bool) -> IoResult<i64> {
            Ok(self.metadata_ts(traverse)?.mtime_nsec())
        }
        /// Returns the last status change time of the file, in seconds since Unix Epoch.
        pub fn ctime_ts(&self, traverse: bool) -> IoResult<i64> {
            Ok(self.metadata_ts(traverse)?.ctime())
        }
        /// Returns the last status change time of the file, in nanoseconds since ctime.
        pub fn ctime_nsec_ts(&self, traverse: bool) -> IoResult<i64> {
            Ok(self.metadata_ts(traverse)?.ctime_nsec())
        }
        /// Returns the block size for filesystem I/O.
        pub fn blksize_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.blksize())
        }
        /// Returns the number of blocks allocated to the file, in 512-byte units.
        pub fn blocks_ts(&self, traverse: bool) -> IoResult<u64> {
            Ok(self.metadata_ts(traverse)?.blocks())
        }
    }
}

// https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html
// https://doc.rust-lang.org/std/fs/struct.Metadata.html#impl-MetadataExt-3
#[cfg(any(windows, doc))]
#[rustfmt::skip]
mod windows {
    use super::FsPath;
    use crate::IoResult;

    #[cfg(windows)]
    use std::os::windows::fs::MetadataExt;
    // Mockup:
    #[cfg(not(windows))]
    crate::items! {
        // https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html
        #[allow(missing_docs, dead_code)]
        trait MetadataExt {
            fn file_attributes(&self) -> u32 {0}
            fn creation_time(&self) -> u64 {0}
            fn last_access_time(&self) -> u64 {0}
            fn last_write_time(&self) -> u64 {0}
            fn file_size(&self) -> u64 {0}
            fn volume_serial_number(&self) -> Option<u32> {None}
            fn number_of_links(&self) -> Option<u32> {None}
            fn file_index(&self) -> Option<u64> {None}
            fn change_time(&self) -> Option<u64> {None}
        }
        impl MetadataExt for crate::FileMetadata {}
    }

    /// # `windows` methods
    impl FsPath {
        /// Returns the value of the dwFileAttributes field of this metadata.
        pub fn file_attributes(&self) -> IoResult<u32> { Ok(self.metadata()?.file_attributes()) }
        /// Returns the value of the ftCreationTime field of this metadata.
        pub fn creation_time(&self) -> IoResult<u64> { Ok(self.metadata()?.creation_time()) }
        /// Returns the value of the ftLastAccessTime field of this metadata.
        pub fn last_access_time(&self) -> IoResult<u64> { Ok(self.metadata()?.last_access_time()) }
        /// Returns the value of the ftLastWriteTime field of this metadata.
        pub fn last_write_time(&self) -> IoResult<u64> { Ok(self.metadata()?.last_write_time()) }
        /// Returns the value of the nFileSize{High,Low} fields of this metadata.
        pub fn file_size(&self) -> IoResult<u64> { Ok(self.metadata()?.file_size()) }
    }
}

#[rustfmt::skip]
mod std_impls {
    use super::{FsPath, PathBuf};
    use crate::Path;

    // From<PathBuf>
    impl From<PathBuf> for FsPath { fn from(path: PathBuf) -> Self { Self { path } } }
    impl From<&PathBuf> for FsPath { fn from(path: &PathBuf) -> Self { Self::new(path) } }
    impl From<&mut PathBuf> for FsPath { fn from(path: &mut PathBuf) -> Self { Self::new(path) } }
    // Into<PathBuf>
    impl From<FsPath> for PathBuf { fn from(fspath: FsPath) -> Self { fspath.path } }
    impl From<&FsPath> for PathBuf { fn from(fspath: &FsPath) -> Self { fspath.path.clone() } }
    impl From<&mut FsPath> for PathBuf { fn from(fspath: &mut FsPath)
        -> Self { fspath.path.clone() } }
    impl<'a> From<&'a FsPath> for &'a PathBuf { fn from(fspath: &'a FsPath)
        -> Self { &fspath.path } }
    impl<'a> From<&'a mut FsPath> for &'a mut PathBuf { fn from(fspath: &'a mut FsPath)
        -> Self { &mut fspath.path } }
    // From<Path>
    impl From<&Path> for FsPath { fn from(path: &Path) -> Self { Self::new(path) } }
    impl From<&mut Path> for FsPath { fn from(path: &mut Path) -> Self { Self::new(path) } }
    // Into<Path>
    impl<'a> From<&'a FsPath> for &'a Path { fn from(fspath: &'a FsPath)
        -> Self { fspath.path.as_path() } }
    impl<'a> From<&'a mut FsPath> for &'a Path { fn from(fspath: &'a mut FsPath)
        -> Self { fspath.path.as_path() } }
}
