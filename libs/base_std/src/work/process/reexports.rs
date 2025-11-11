// devela_base_std::work::process::reexports
//
//!
//
// Note that std's standalone functions are namespaced in `devela::ProcessExt`.
//
// WAIT: [exit_status_error](https://github.com/rust-lang/rust/issues/84908)

use crate::_reexport;

/* traits */

_reexport! { rust: std::process,
    doc: "A trait for implementing arbitrary return types in the `main` function.",
    @Termination as ProcessTermination
}

/* types */

_reexport! { rust: std::process,
    doc: "Representation of a running or exited child process.\n\n
See also the [`ProcessExt`][crate::ProcessExt] trait.",
    @Child as Process
}
_reexport! { rust: std::process,
    doc: "A handle to a child process’s stderr.",
    @ChildStderr as ProcessStderr
}
_reexport! { rust: std::process,
    doc: "A handle to a child process’s standard input (stdin).",
    @ChildStdin as ProcessStdin
}
_reexport! { rust: std::process,
    doc: "A handle to a child process’s standard output (stdout).",
    @ChildStdout as ProcessStdout
}
_reexport! { rust: std::process,
    doc: "A builder for configuring and spawning new processes.",
    @Command as ProcessCommand
}
_reexport! { rust: std::process,
    doc: "An iterator over the command arguments.",
    @CommandArgs as ProcessCommandArgs
}
_reexport! { rust: std::process,
    doc: "An iterator over the command environment variables.",
    @CommandEnvs as ProcessCommandEnvs
}
_reexport! { rust: std::process,
    tag: crate::_TAG_RESULT!(),
    doc: "The status code the process returns to its parent on normal termination.",
    @ExitCode as ProcessExitCode
}
_reexport! { rust: std::process,
    tag: crate::_TAG_RESULT!(),
    doc: "Describes the result of a process after it has terminated.",
    @ExitStatus as ProcessExitStatus
}
_reexport! { rust: std::process,
    doc: "The output of a finished process.",
    @Output as ProcessOutput
}
_reexport! { rust: std::process,
    doc: "Specifies how to handle standard I/O streams in [`ProcessCommand`].",
    @Stdio as ProcessStdio
}
