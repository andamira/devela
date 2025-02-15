// devela::sys::fs::namespace
//
//! `Fs` namespace.
//

use crate::{
    FileMetadata, FilePermissions, IoResult, IterDirRead, Path, PathBuf,
    _dep::_std::fs::{
        canonicalize, copy, create_dir, create_dir_all, exists, hard_link, metadata, read,
        read_dir, read_link, read_to_string, remove_dir, remove_dir_all, remove_file, rename,
        set_permissions, symlink_metadata, write,
    },
};

#[doc = crate::TAG_NAMESPACE!()]
/// Filesystem-related operations.
///
/// See also: [`ExtPath`][crate::ExtPath], [`fsPath`][crate::FsPath].
pub struct Fs;

/// # Safe methods.
#[rustfmt::skip]
impl Fs {
    /// Returns the canonical, absolute form of a path with all intermediate components normalized
    /// and symbolic links resolved.
    ///
    /// See `std::sys::`[`canonicalize`].
    pub fn canonicalize<P: AsRef<Path>>(path: P) -> IoResult<PathBuf> { canonicalize(path) }

    /// Copies the contents of one file to another. This function will also copy the permission
    /// bits of the original file to the destination file.
    ///
    /// See `std::sys::`[`copy`].
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> IoResult<u64> { copy(from, to) }

    /// Creates a new, empty directory at the provided path
    ///
    /// See `std::sys::`[`create_dir`].
    pub fn create_dir<P: AsRef<Path>>(path: P) -> IoResult<()> { create_dir(path) }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// See `std::sys::`[`create_dir_all`].
    pub fn create_dir_all<P: AsRef<Path>>(path: P) -> IoResult<()> { create_dir_all(path) }

    /// Returns Ok(true) if the path points at an existing entity.
    ///
    /// See `std::sys::`[`exists`].
    pub fn exists<P: AsRef<Path>>(path: P) -> IoResult<bool> { exists(path) }

    /// Creates a new hard link on the filesystem.
    ///
    /// See `std::sys::`[`hard_link`].
    pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> IoResult<()> {
        hard_link(original, link)
    }

    /// Given a path, queries the file system to get information about a file, directory, etc.
    ///
    /// See `std::sys::`[`metadata`].
    pub fn metadata<P: AsRef<Path>>(path: P) -> IoResult<FileMetadata> { metadata(path) }

    /// Reads the entire contents of a file into a bytes vector.
    ///
    /// See `std::sys::`[`read`].
    pub fn read<P: AsRef<Path>>(path: P) -> IoResult<Vec<u8>> { read(path) }

    /// Returns an iterator over the entries within a directory.
    ///
    /// See `std::sys::`[`read_dir`].
    pub fn read_dir<P: AsRef<Path>>(path: P) -> IoResult<IterDirRead> { read_dir(path) }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// See `std::sys::`[`read_link`].
    pub fn read_link<P: AsRef<Path>>(path: P) -> IoResult<PathBuf> { read_link(path) }

    /// Reads the entire contents of a file into a string.
    ///
    /// See `std::sys::`[`read_to_string`].
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> IoResult<String> { read_to_string(path) }

    /// Removes an empty directory.
    ///
    /// See `std::sys::`[`remove_dir`].
    pub fn remove_dir<P: AsRef<Path>>(path: P) -> IoResult<()> { remove_dir(path) }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    /// See `std::sys::`[`remove_dir_all`].
    pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> IoResult<()> { remove_dir_all(path) }

    /// Removes a file from the filesystem.
    ///
    /// See `std::sys::`[`remove_file`].
    pub fn remove_file<P: AsRef<Path>>(path: P) -> IoResult<()> { remove_file(path) }

    /// Renames a file or directory to a new name, replacing the original file if to already
    /// exists.
    ///
    /// See `std::sys::`[`rename`].
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> IoResult<()> {
        rename(from, to)
    }

    /// Changes the permissions found on a file or a directory.
    ///
    /// See `std::sys::`[`set_permissions`].
    pub fn set_permissions<P: AsRef<Path>>(path: P, perm: FilePermissions) -> IoResult<()> {
        set_permissions(path, perm)
    }

    /// Queries the metadata about a file without following symlinks.
    ///
    /// See `std::sys::`[`symlink_metadata`].
    pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> IoResult<FileMetadata> {
        symlink_metadata(path)
    }

    /// Writes a slice as the entire contents of a file.
    ///
    /// See `std::sys::`[`fn@write`].
    pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> IoResult<()> {
        write(path, contents)
    }
}
