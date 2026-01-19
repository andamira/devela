// devela_base_std::work::process::cmd
//
//! Defines [`cmd!`].
//

#[doc = crate::_tags!(code platform runtime)]
/// Builds a [`Pipeline`] from a linear sequence of commands.
#[doc = crate::_doc_location!("work/process")]
///
/// Grammar (informal):
/// - A pipeline is one or more *segments* separated by `=>`.
/// - Each segment is a command invocation:
///   - The first expression is the program.
///   - Remaining expressions are arguments to that program.
/// - A single segment is treated as a pipeline of length 1.
///
/// Notes:
/// - This macro does not invoke a shell.
/// - No redirection, globbing, or quoting is performed.
/// - Argument splitting only happens when a segment consists of a single string literal.
///
/// # Example
/// ```
/// # use devela::{cmd};
/// # #[cfg(not(miri))] {
/// let arg1 = "-F";
/// let cmd2 = "grep";
///
/// // a single command. E.g.: `ls -F .`
/// cmd!("ls").run();
/// cmd!("ls", arg1, ".").run();
/// cmd!("ls -F .").run();
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
macro_rules! cmd {
    // Entry point for single-command pipelines.
    //   cmd!(a, b)
    ($($args:expr),+ $(,)?) => {{
        $crate::Pipeline::new($crate::cmd!(%cmd $($args),+))
    }};
    // Entry point for multi-command pipelines.
    //   cmd!(a, b => c, d => e)
    ($($first:expr),+ $(=> $($rest:expr),+ )+) => {{
        let p = $crate::Pipeline::new($crate::cmd!(%cmd $($first),+)); // build the first
        $crate::cmd!(%pipe p $(=> $($rest),+ )+) // fold the rest
    }};
    // unsupported cases:
    () => { compile_error!("`cmd!` needs at least one command"); };
    //
    // Recursively folds => separated segments into a linear pipeline.
    (%pipe $pipeline:expr => $($next:expr),+ $(=> $($rest:expr),+ )*) => {{
        let p = $pipeline.then($crate::cmd!(%cmd $($next),+)); // append the next
        $crate::cmd!(%pipe p $(=> $($rest),+ )*) // repeat for the rest
    }};
    // Stops the recursive pipeline fold and returns the fully built pipeline.
    (%pipe $pipeline:expr) => { $pipeline };
    // Command constructor: A single string literal is split on whitespace.
    (%cmd $cmd:literal) => {{
        let mut it = $cmd.split_whitespace();
        let prog = it.next().expect("empty command literal");
        let mut c = $crate::Command::new(prog);
        for arg in it { c.arg(arg); }
        c
    }};
    // Command constructor: First expr is the program, the rest are arguments.
    (%cmd $prog:expr $(, $arg:expr)* $(,)?) => {{
        let mut c = $crate::Command::new($prog); $( c.arg($arg); )* c
    }};
}
