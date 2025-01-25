// devela::work::process::reexports
//
//! Reexported items.
//
// Note that std's standalone functions are namespaced in `ExtProcess`.
//
// WAIT: [exit_status_error](https://github.com/rust-lang/rust/issues/84908)

use crate::reexport;

/* traits */

reexport! { rust: std::process,
    doc: "A trait for implementing arbitrary return types in the `main` function.",
    @Termination as ProcessTermination
}

/* types */

reexport! { rust: std::process,
    doc: "Representation of a running or exited child process.\n\n
See also the [`ExtProcess`][crate::ExtProcess] trait.",
    @Child as Process
}
reexport! { rust: std::process,
    doc: "A handle to a child process’s stderr.",
    @ChildStderr as ProcessStderr
}
reexport! { rust: std::process,
    doc: "A handle to a child process’s standard input (stdin).",
    @ChildStdin as ProcessStdin
}
reexport! { rust: std::process,
    doc: "A handle to a child process’s standard output (stdout).",
    @ChildStdout as ProcessStdout
}
reexport! { rust: std::process,
    doc: "A builder for configuring and spawning new processes.",
    @Command as ProcessCommand
}
reexport! { rust: std::process,
    doc: "An iterator over the command arguments.",
    @CommandArgs as ProcessCommandArgs
}
reexport! { rust: std::process,
    doc: "An iterator over the command environment variables.",
    @CommandEnvs as ProcessCommandEnvs
}
reexport! { rust: std::process,
    tag: crate::TAG_RESULT!(),
    doc: "The status code the process returns to its parent on normal termination.",
    @ExitCode as ProcessExitCode
}
reexport! { rust: std::process,
    tag: crate::TAG_RESULT!(),
    doc: "Describes the result of a process after it has terminated.",
    @ExitStatus as ProcessExitStatus
}
reexport! { rust: std::process,
    doc: "The output of a finished process.",
    @Output as ProcessOutput
}
reexport! { rust: std::process,
    doc: "Specifies how to handle standard I/O streams in [`ProcessCommand`].",
    @Stdio as ProcessStdio
}
