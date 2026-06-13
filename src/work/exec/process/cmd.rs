// devela/src/work/exec/process/cmd.rs
//
//! Defines [`cmd!`].
//

#[doc = crate::_tags!(code platform runtime)]
/// Builds a [`CommandFlow`][crate::CommandFlow] from one or more command invocations.
#[doc = crate::_doc_meta!{location("work/process")}]
///
/// Grammar (informal):
/// - The `=>` operator constructs a linear flow,
///   connecting each command's stdout to the stdin of the next.
/// - Each direct command segment consists of:
///   - The first expression as the program.
///   - Any remaining expressions as arguments to that program.
/// - Prefixing a string literal with `@` splits it
///   into a program and arguments using shell word syntax.
/// - A single command segment forms a command flow of length 1.
///
/// Semantics:
/// - This macro does not invoke a shell.
/// - Commands are spawned directly through the OS process API.
/// - Direct segments pass their expressions unchanged as argv words.
/// - `@` segments only perform shell-style word splitting and quoting.
/// - No variable expansion, globbing, redirection, command substitution,
///   or shell operators are performed.
///
/// The `@` syntax requires the `shell` feature.
///
/// # Examples
/// ```
/// # use devela::cmd;
/// # #[cfg(not(miri))] {
/// let arg1 = "-F";
/// let cmd2 = "grep";
///
/// // A single direct command.
/// cmd!("ls").run();
/// cmd!("ls", arg1, ".").run();
/// cmd!("ls", "-F", ".").run();
///
/// // A literal split into a program and arguments.
/// #[cfg(feature = "shell")]
/// cmd!(@ "ls -F .").run();
///
/// // Quoting controls argument boundaries.
/// #[cfg(feature = "shell")]
/// cmd!(@ r#"echo "hello world""#).run();
///
/// // Shell expansion is not performed.
/// #[cfg(feature = "shell")]
/// cmd!(@ r#"echo "$HOME" "*.rs""#).run(); // literal "$HOME" and "*.rs"
///
/// // Multiple piped commands: `ps aux | grep lib | wc -l`.
/// cmd!("ps", "aux" => cmd2, "lib" => "wc", "-l").run();
///
/// #[cfg(feature = "shell")]
/// cmd!(@ "ps aux" => @ "grep lib" => @ "wc -l").run();
///
/// // Direct and split segments may be combined.
/// #[cfg(feature = "shell")]
/// cmd!("printf", "hello world" => @ r#"grep "hello world""# => "wc", "-l").run();
/// # }
/// ```
///
/// # No implicit splitting
///
/// Without `@`, a string is treated as one argv word:
/// ```no_run
/// # use devela::cmd;
/// # #[cfg(not(miri))] {
/// cmd!("ls -F").run();      // executes a program named `ls -F`
/// cmd!("ls -F", ".").run(); // executes `ls -F` with `.` as its argument
/// # }
/// ```
/// Use `cmd!(@ "ls -F")` when shell-word splitting is intended.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cmd {
    // Entry point for a shell-parsed single-command flow.
    //   cmd!(@ "program arg")
    (@ $cmd:literal $(,)?) => {{
        $crate::CommandFlow::new($crate::cmd!(%cmd @ $cmd))
    }};
    // Entry point for a shell-parsed first command in a multi-command flow.
    //   cmd!(@ "a b" => c, d)
    (@ $first:literal => $($rest:tt)+) => {{
        let flow = $crate::CommandFlow::new($crate::cmd!(%cmd @ $first));
        $crate::cmd!(%flow flow => $($rest)+)
    }};
    // Entry point for a direct multi-command flow.
    //   cmd!(a, b => c, d => e)
    ($($first:expr),+ => $($rest:tt)+) => {{
        let flow = $crate::CommandFlow::new($crate::cmd!(%cmd $($first),+));
        $crate::cmd!(%flow flow => $($rest)+)
    }};
    // Entry point for a direct single-command flow.
    //   cmd!(a)
    //   cmd!(a, b)
    ($($args:expr),+ $(,)?) => {{
        $crate::CommandFlow::new($crate::cmd!(%cmd $($args),+))
    }};
    // Unsupported case.
    () => { compile_error!("`cmd!` needs at least one command") };
    // Appends a shell-parsed command and continues folding.
    (%flow $flow:expr => @ $next:literal => $($rest:tt)+) => {{
        let flow = $flow.then($crate::cmd!(%cmd @ $next));
        $crate::cmd!(%flow flow => $($rest)+)
    }};
    // Appends the final shell-parsed command.
    (%flow $flow:expr => @ $next:literal $(,)?) => {{
        $flow.then($crate::cmd!(%cmd @ $next))
    }};
    // Appends a direct command and continues folding.
    (%flow $flow:expr => $($next:expr),+ => $($rest:tt)+) => {{
        let flow = $flow.then($crate::cmd!(%cmd $($next),+));
        $crate::cmd!(%flow flow => $($rest)+)
    }};
    // Appends the final direct command.
    (%flow $flow:expr => $($next:expr),+ $(,)?) => {{
        $flow.then($crate::cmd!(%cmd $($next),+))
    }};
    // Constructs a command by splitting one literal with shell-word rules.
    (%cmd @ $cmd:literal) => {{
        $crate::__cmd_shell!($cmd)
    }};
    // Constructs a command directly from a program and zero or more arguments.
    (%cmd $prog:expr $(, $arg:expr)* $(,)?) => {{
        let mut command = $crate::Command::new($prog);
        $(command.arg($arg);)*
        command
    }};
}
#[doc(inline)]
pub use cmd;

#[cfg(feature = "shell")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cmd_shell {
    ($cmd:literal) => {{
        use $crate::ProcessExt as _;
        $crate::Process::command_shell($cmd).expect("invalid command literal")
    }};
}
#[cfg(not(feature = "shell"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cmd_shell {
    ($cmd:literal) => {{ compile_error!("`cmd!(@ \"...\")` requires devela's `shell` feature") }};
}
#[doc(hidden)]
pub use __cmd_shell;
