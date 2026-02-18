// devela_base_std::work::process::flow
//
//! Defines [`CommandFlow`].
//

use crate::sys::io::{Io, IoRead, IoResult};
use crate::work::process::{Command, ExitStatus, Output, Stdio};

#[doc = crate::_tags!(platform runtime)]
/// An executable flow of OS process invocations.
#[doc = crate::_doc_location!("work/process")]
///
/// A `CommandFlow` represents one or more [`Command`]s executed as a single unit.
/// In the current implementation, commands are arranged linearly, connecting
/// the stdout of each command to the stdin of the next.
///
/// Command flows are constructed programmatically (for example via [`cmd!`][crate::cmd])
/// and do not involve a shell. As a result:
/// - No globbing, quoting, or redirection is performed.
/// - Each command is spawned directly via the OS process API.
///
/// Execution is eager: all commands in the flow are spawned before waiting for completion.
///
/// See also the [`cmd!`][crate::cmd] macro for a compact construction syntax.
#[derive(Debug)]
pub struct CommandFlow {
    cmds: Vec<Command>,
}
#[rustfmt::skip]
impl CommandFlow {
    /// Creates a new flow with a single command.
    ///
    /// This is equivalent to a flow of length 1, with no piping.
    pub fn new(cmd: Command) -> Self { Self { cmds: vec![cmd] } }

    /// Appends a command to the flow, piping the previous command into it.
    ///
    /// The stdout of the current command is connected to the stdin of `next` using an OS pipe.
    ///
    /// This consumes the flow and returns an extended one, allowing chaining.
    ///
    /// # Panics
    /// Panics if the flow is empty. This invariant is upheld by construction.
    pub fn then(mut self, mut next: Command) -> Self {
        let (reader, writer) = Io::pipe().expect("pipe failed");
        let left = self.cmds.last_mut().unwrap();
        left.stdout(Stdio::from(writer));
        next.stdin(Stdio::from(reader));
        self.cmds.push(next);
        self
    }

    /// Runs the flow and waits for all commands to complete.
    ///
    /// All commands are spawned first. The method then waits for each process,
    /// returning the exit status of the **last** command in the flow.
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

    /// Runs the flow and captures the standard output of the last command.
    ///
    /// The stdout of the final command is redirected into a pipe and fully
    /// collected into a byte buffer.
    ///
    /// All commands are spawned before any output is read.
    /// The method waits for all commands to complete and returns
    /// an [`Output`] containing the exit status and captured stdout.
    /// The stderr field is empty.
    ///
    /// # Notes
    /// - Output is buffered in memory.
    /// - Stderr is not captured.
    ///
    /// # Errors
    /// Returns an I/O error if spawning, piping, reading, or waiting fails.
    pub fn stdout(mut self) -> IoResult<Output> {
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
        let mut last_status = ExitStatus::default();
        for mut child in children { last_status = child.wait()?; }
        Ok(Output { status: last_status, stdout: output, stderr: Vec::new() })
    }

    /// Runs the flow and captures both stdout and stderr of the last command.
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
    /// Returns an [`Output`] containing the exit status and merged stdout/stderr.
    /// The merged output is stored in `Output.stdout`; `stderr` is empty.
    ///
    /// # Errors
    /// Returns an I/O error if spawning, piping, reading, or waiting fails.
    pub fn output(mut self) -> IoResult<Output> {
        let (mut reader, writer) = Io::pipe()?;
        let last = self.cmds.last_mut().unwrap();
        // fan-in: clone writer
        last.stdout(Stdio::from(writer.try_clone()?));
        last.stderr(Stdio::from(writer));
        let mut children = Vec::with_capacity(self.cmds.len());
        for mut cmd in self.cmds { children.push(cmd.spawn()?); }
        let mut output = Vec::new();
        reader.read_to_end(&mut output)?;
        let mut last_status = ExitStatus::default();
        for mut child in children { last_status = child.wait()?; }
        Ok(Output { status: last_status, stdout: output, stderr: Vec::new() })
    }

    /// Runs the flow and streams the stdout of the last command to a callback.
    ///
    /// The provided function `f` is invoked incrementally with chunks of bytes
    /// as they become available from the final command's standard output.
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
