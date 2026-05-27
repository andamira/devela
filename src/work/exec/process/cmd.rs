// devela::work::exec::process::cmd
//
//! Defines [`cmd!`].
//

#[doc = crate::_tags!(code platform runtime)]
/// Builds a [`CommandFlow`][crate::CommandFlow] from one or more command invocations.
#[doc = crate::_doc_meta!{location("work/process")}]
///
/// Grammar (informal):
/// - The `=>` operator constructs a linear flow, connecting each command's
///   stdout to the stdin of the next.
/// - Each segment is a command invocation:
///   - The first expression is the program.
///   - Remaining expressions are arguments to that program.
/// - A single segment is treated as a command flow of length 1.
///
/// Semantics:
/// - This macro does not invoke a shell.
/// - Commands are spawned directly through the OS process API.
/// - No expansion, globbing, redirection, variables, or pipes are performed.
/// - Explicit segments like `cmd!("echo", "hello world")` are passed as argv words.
/// - Single string-literal segments like `cmd!("echo 'hello world'")` require
///   the `shell` feature and are split using shell word syntax.
///
/// # Examples
/// ```
/// # use devela::{cmd};
/// # #[cfg(not(miri))] {
/// let arg1 = "-F";
/// let cmd2 = "grep";
///
/// // a single command. E.g.: `ls -F .`
/// cmd!("ls").run();
/// cmd!("ls", arg1, ".").run();
/// cmd!("ls", "-F", ".").run();
///
/// #[cfg(feature = "shell")]
/// cmd!("ls -F .").run();
///
/// #[cfg(feature = "shell")]
/// cmd!(r#"echo "hello world""#).run();
///
/// // Still no shell expansion:
/// #[cfg(feature = "shell")]
/// cmd!(r#"echo "$HOME" "*.rs""#).run(); // literal "$HOME" and "*.rs"
///
/// // multiple piped commands. E.g.: `ps aux | grep lib | wc -l`
/// cmd!("ps", "aux" => cmd2, "lib" => "wc", "-l").run();
/// cmd!("ps aux" => "grep lib" => "wc -l").run();
///
/// // NOTE: This is invalid. The program will be treated as `"ls -F"`, not `"ls"`,
/// // because splitting only happens when the *entire segment* is a single literal:
/// // cmd!("ls -F", "."); // ❌ invalid, no splitting will happen
/// # }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cmd {
    // Entry point for a single-command flow.
    //   cmd!(a, b)
    ($($args:expr),+ $(,)?) => {{
        $crate::CommandFlow::new($crate::cmd!(%cmd $($args),+))
    }};
    // Entry point for a multi-command flow.
    //   cmd!(a, b => c, d => e)
    ($($first:expr),+ $(=> $($rest:expr),+ )+) => {{
        let p = $crate::CommandFlow::new($crate::cmd!(%cmd $($first),+)); // build the first
        $crate::cmd!(%flow p $(=> $($rest),+ )+) // fold the rest
    }};
    // unsupported cases:
    () => { compile_error!("`cmd!` needs at least one command"); };
    //
    // Recursively folds => separated segments into a command flow.
    (%flow $flow:expr => $($next:expr),+ $(=> $($rest:expr),+ )*) => {{
        let p = $flow.then($crate::cmd!(%cmd $($next),+)); // append the next
        $crate::cmd!(%flow p $(=> $($rest),+ )*) // repeat for the rest
    }};
    // Stops the recursive fold and returns the fully built command flow.
    (%flow $flow:expr) => { $flow };
    // Command constructor: A single string literal is split on whitespace.
    (%cmd $cmd:literal) => {{
        #[cfg(feature = "shell")]
        {
            use $crate::ProcessExt as _;
            $crate::Process::command_shell($cmd).expect("invalid command literal")
        }
        #[cfg(not(feature = "shell"))]
        {
            compile_error!(
                "`cmd!(\"...\")` single-literal splitting requires the `shell` feature; \
                 use `cmd!(program, args...)` or enable `shell`"
            );
        }
    }};
    // Command constructor: First expr is the program, the rest are arguments.
    (%cmd $prog:expr $(, $arg:expr)* $(,)?) => {{
        let mut c = $crate::Command::new($prog); $( c.arg($arg); )* c
    }};
}
#[doc(inline)]
pub use cmd;
