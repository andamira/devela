// devela::sys::os::linux::namespace::definition
//
//! Defines the [`Linux`] namespace.
//

#[doc = crate::_tags!(linux namespace)]
/// Linux-related operations.
#[doc = crate::_doc_meta!{location("sys/os/linux")}]
///
/// # Features
/// All the methods depend on the features `linux` and `unsafe_syscall`.
///
/// # Methods
/// - [read](#read-related-methods)
/// - [write](#write-related-methods)
/// - [term](#terminal-related-methods)
/// - [thread](#thread-related-methods)
/// - [signal](#signaling-related-methods)
/// - [random](#randomness-related-methods)
/// - syscalls:
///   - [file-descriptors](#syscalls-file-descriptors)
///   - [filesystem](#syscalls-filesystem)
///   - [device and special I/O](#syscalls-device-and-special-io)
///   - [IPC](#syscalls-ipc)
///   - [process control](#syscalls-process-control)
///   - [timing and signal handling](#syscalls-timing-and-signal-handling)
#[derive(Debug)]
pub struct Linux;
