// devela_base_std::work::process::_reexport
//
//!
//
// Note that std's standalone functions are namespaced in `devela::ProcessExt`.
//
// WAIT: [exit_status_error](https://github.com/rust-lang/rust/issues/84908)

use crate::{_reexport, _tags};

/* traits */

_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime),
    doc: "A trait for implementing arbitrary return types in the `main` function.",
    @Termination as ProcessTermination
}

/* types */

_reexport! { rust: std::process, location: "work/process", tag: _tags!(concurrency),
    doc: "Representation of a running or exited child process.",
    +doc: "See also the [`ProcessExt`][crate::ProcessExt] trait.",
    @Child as Process
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(io),
    doc: "A handle to a child process's stderr.",
    @ChildStderr as ProcessStderr
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(io),
    doc: "A handle to a child process's standard input (stdin).",
    @ChildStdin as ProcessStdin
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(io),
    doc: "A handle to a child process's standard output (stdout).",
    @ChildStdout as ProcessStdout
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime),
    doc: "A builder for configuring and spawning new processes.",
    Command
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime),
    doc: "An iterator over the command arguments.",
    CommandArgs
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime),
    doc: "An iterator over the command environment variables.",
    CommandEnvs
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime result),
    doc: "The status code the process returns to its parent on normal termination.",
    ExitCode
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime result),
    doc: "Describes the result of a process after it has terminated.",
    ExitStatus
}
_reexport! { rust: std::process, location: "work/process", tag: _tags!(platform runtime),
    doc: "The output of a finished process.",
    Output
}
_reexport! { rust: std::process, location: "work/process", tag:  _tags!(platform runtime io),
    doc: "Specifies how to handle standard I/O streams in [`ProcessCommand`].",
    Stdio
}
