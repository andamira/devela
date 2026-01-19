// devela_base_std::work::process::pipe
//
//! Defines [`Pipeline`].
//

use crate::{Command, ExitStatus, Stdio};
use crate::{Io, IoError, IoErrorKind, IoRead, IoResult};

#[doc = crate::_tags!(platform runtime)]
/// A linear sequence of OS process invocations connected by pipes.
#[doc = crate::_doc_location!("work/process")]
///
/// A `Pipeline` represents one or more [`Command`]s executed in order, where the
/// standard output of each command is connected to the standard input of the next.
///
/// Pipelines are constructed programmatically (for example via [`cmd!`]) and
/// do not involve a shell. As a result:
/// - No globbing, quoting, or redirection is performed.
/// - Each command is spawned directly via the OS process API.
///
/// Execution is eager: all commands in the pipeline are spawned before waiting for completion.
///
/// See also the [`cmd!`][crate::cmd] macro for a compact pipeline syntax.
#[derive(Debug)]
pub struct Pipeline {
    cmds: Vec<Command>,
}
#[rustfmt::skip]
impl Pipeline {
    /// Creates a new pipeline with a single command.
    ///
    /// This is equivalent to a pipeline of length 1, with no piping.
    /// Additional commands can be appended using [`Pipeline::then`].
    pub fn new(cmd: Command) -> Self { Self { cmds: vec![cmd] } }

    /// Appends a command to the pipeline, piping the previous command into it.
    ///
    /// The stdout of the current command is connected to the stdin of `next` using an OS pipe.
    ///
    /// This consumes the pipeline and returns an extended one, allowing chaining.
    ///
    /// # Panics
    /// Panics if the pipeline is empty. This invariant is upheld by construction.
    pub fn then(mut self, mut next: Command) -> Self {
        let (reader, writer) = Io::pipe().expect("pipe failed");
        let left = self.cmds.last_mut().unwrap();
        left.stdout(Stdio::from(writer));
        next.stdin(Stdio::from(reader));
        self.cmds.push(next);
        self
    }

    /// Runs the pipeline and waits for all commands to complete.
    ///
    /// All commands are spawned first. The method then waits for each process,
    /// returning the exit status of the **last** command in the pipeline.
    ///
    /// Standard input, output, and error streams are inherited unless modified
    /// earlier (for example by piping or capture methods).
    ///
    /// # Errors
    /// Returns an I/O error if any command fails to spawn or wait.
    pub fn run(self) -> IoResult<ExitStatus> {
        let mut children = Vec::with_capacity(self.cmds.len());
        for mut cmd in self.cmds { children.push(cmd.spawn()?); }
        let mut last_status = None;
        for mut child in children { last_status = Some(child.wait()?); }
        Ok(last_status.unwrap())
    }

    /// Runs the pipeline and captures the standard output of the last command.
    ///
    /// The stdout of the final command is redirected into a pipe and fully
    /// collected into a byte buffer.
    ///
    /// All commands are spawned before any output is read.
    /// The method waits for all commands to complete and returns:
    /// - the exit status of the last command
    /// - the captured stdout bytes
    ///
    /// # Notes
    /// - Output is buffered in memory.
    /// - Stderr is not captured.
    ///
    /// # Errors
    /// Returns an I/O error if spawning, piping, reading, or waiting fails.
    pub fn stdout(mut self) -> IoResult<(ExitStatus, Vec<u8>)> {
        let (mut reader, writer) = Io::pipe()?;
        // attach writer to last command's stdout
        let last = self.cmds.last_mut().unwrap();
        last.stdout(Stdio::from(writer));
        // spawn all commands
        let mut children = Vec::with_capacity(self.cmds.len());
        for mut cmd in self.cmds { children.push(cmd.spawn()?); }
        // read before waiting
        let mut output = Vec::new();
        reader.read_to_end(&mut output)?;
        // wait for children
        let mut last_status = None;
        for mut child in children { last_status = Some(child.wait()?); }
        Ok((last_status.unwrap(), output))
    }

    /// Runs the pipeline and returns the last command’s stdout as a UTF-8 string.
    ///
    /// This is a convenience wrapper over [`Pipeline::stdout`], converting the
    /// captured stdout bytes into a [`String`] using strict UTF-8 decoding.
    ///
    /// # Errors
    /// - Returns an I/O error if the pipeline fails to execute or capture output.
    /// - Returns [`InvalidData`] if stdout is not valid UTF-8.
    ///
    /// The exit status of the last command is returned alongside the decoded string.
    pub fn stdout_string(self) -> IoResult<(ExitStatus, String)> {
        let (status, bytes) = self.stdout()?;
        let str = String::from_utf8(bytes).map_err(|e| IoError::new(IoErrorKind::InvalidData, e))?;
        Ok((status, str))
    }

    /// Runs the pipeline and returns the last command’s stdout as a UTF-8 string,
    /// replacing invalid sequences.
    ///
    /// This method is identical to [`stdout_string`], except that invalid UTF-8 sequences
    /// are replaced with the Unicode replacement character (�) instead of causing an error.
    pub fn stdout_string_lossy(self) -> IoResult<(ExitStatus, String)> {
        let (status, bytes) = self.stdout()?;
        let str = String::from_utf8_lossy(&bytes);
        Ok((status, str.to_string()))
    }

    /// Runs the pipeline and captures both stdout and stderr of the last command.
    ///
    /// The stdout and stderr streams of the final command are merged
    /// into a single byte stream, equivalent to:
    /// ```text
    /// ... | cmd 2>&1
    /// ```
    ///
    /// # Notes
    /// - The relative ordering of stdout and stderr bytes is unspecified.
    /// - Output is buffered in memory.
    ///
    /// Returns the exit status of the last command and the merged output bytes.
    ///
    /// # Errors
    /// Returns an I/O error if spawning, piping, reading, or waiting fails.
    pub fn output(mut self) -> IoResult<(ExitStatus, Vec<u8>)> {
        let (mut reader, writer) = Io::pipe()?;
        let last = self.cmds.last_mut().unwrap();
        // fan-in: clone writer
        last.stdout(Stdio::from(writer.try_clone()?));
        last.stderr(Stdio::from(writer));
        let mut children = Vec::with_capacity(self.cmds.len());
        for mut cmd in self.cmds { children.push(cmd.spawn()?); }
        let mut output = Vec::new();
        reader.read_to_end(&mut output)?;
        let mut last_status = None;
        for mut child in children { last_status = Some(child.wait()?); }
        Ok((last_status.unwrap(), output))
    }

    /// Runs the pipeline and streams the stdout of the last command to a callback.
    ///
    /// The provided function `f` is invoked incrementally with chunks of bytes
    /// as they become available from the final command’s standard output.
    ///
    /// This enables streaming consumption without buffering the full output in memory.
    ///
    /// # Execution model
    /// - All commands are spawned eagerly.
    /// - Bytes are read from the pipe in a loop.
    /// - The callback is invoked on the reader thread.
    ///
    /// # Notes
    /// - Only stdout of the last command is streamed.
    /// - Stderr is inherited.
    ///
    /// Returns the exit status of the last command.
    ///
    /// # Errors
    /// Returns an I/O error if spawning, piping, reading, or waiting fails.
    pub fn stdout_with<F>(mut self, mut f: F) -> IoResult<ExitStatus> where F: FnMut(&[u8]) {
        let (mut reader, writer) = Io::pipe()?;
        let last = self.cmds.last_mut().unwrap();
        last.stdout(Stdio::from(writer));
        let mut children = Vec::with_capacity(self.cmds.len());
        for mut cmd in self.cmds { children.push(cmd.spawn()?); }
        let mut buf = [0u8; 8192];
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 { break; }
            f(&buf[..n]);
        }
        let mut last_status = None;
        for mut child in children { last_status = Some(child.wait()?); }
        Ok(last_status.unwrap())
    }
}
