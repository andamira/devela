// devela::work::exec::process::process
//
//! Defines the [`ProcessExt`] trait.
//
// IMPROVE
// https://doc.rust-lang.org/std/os/linux/process/trait.CommandExt.html
// https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html
// https://doc.rust-lang.org/std/os/windows/process/trait.CommandExt.html

use crate::{Command, OsStr, Process, Str};
#[cfg(feature = "shell")]
use crate::{ShellLex, ShellWordError};
use std::process::{abort, exit, id};

/// Marker trait to prevent downstream implementations of the [`ProcessExt`] trait.
trait Sealed {}
impl Sealed for Process {}

#[doc = crate::_tags!(platform concurrency namespace)]
/// Extension trait providing additional methods for [`Process`]es.
#[doc = crate::_doc_location!("work/process")]
///
/// It offers the standalone functions in `std::process` as associated methods.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ProcessExt: Sealed {
    /// Constructs a new `Command` for launching the `program`.
    ///
    /// See `Command::`[new][Command::new].
    fn command<S: AsRef<OsStr>>(program: S) -> Command {
        Command::new(program)
    }

    /// Constructs a new `Command` from shell-like command words.
    ///
    /// This does not invoke a shell. It only uses shell word syntax to split
    /// `line` into a program and arguments.
    ///
    /// # Errors
    /// Returns an error if `line` does not contain a program word, contains
    /// invalid shell word syntax, or contains a decoded word that is not UTF-8.
    #[cfg(feature = "shell")]
    fn command_shell(line: &str) -> Result<Command, ShellWordError> {
        let mut lex = ShellLex::new(line);
        let mut buf = crate::vec_![0u8; line.len()];
        let Some(len) = lex.next_word_to(&mut buf)? else {
            return Err(ShellWordError::EmptyCommand);
        };
        let prog = Str::from_utf8(&buf[..len]).map_err(|_| ShellWordError::InvalidUtf8)?;
        let mut cmd = Command::new(prog);
        while let Some(len) = lex.next_word_to(&mut buf)? {
            let arg = Str::from_utf8(&buf[..len]).map_err(|_| ShellWordError::InvalidUtf8)?;
            cmd.arg(arg);
        }
        Ok(cmd)
    }

    /// Terminates the current process in an abnormal fashion.
    ///
    /// See `std::process::`[abort].
    #[rustfmt::skip]
    fn abort() -> ! { abort() }

    /// Terminates the current process with the specified exit code.
    ///
    /// See `std::process::`[exit].
    #[rustfmt::skip]
    fn exit(code: i32) -> ! { exit(code) }

    /// Returns the OS-assigned process identifier associated with this process.
    ///
    /// See `std::process::`[id].
    #[must_use] #[rustfmt::skip]
    fn self_pid() -> u32 { id() }
}
impl ProcessExt for Process {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "shell")]
    fn command_ext_command_shell() {
        let cmd = Process::command_shell(r#"echo "hello world""#).unwrap();
        let dbg = format!("{cmd:?}");

        assert!(dbg.contains("echo"));
        assert!(dbg.contains("hello world"));
    }
}
