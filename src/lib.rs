// devela_macros
//
//!
//

extern crate proc_macro;
use proc_macro::TokenStream;

/// Compiles the associated code only if the inner predicate equals `true`.
///
/// Anything other than `true` is considered `false` and the code wont compile.
///
/// It supports the following argument wrapper modifiers, that can be nested:
/// - `not()`: returns `true` if the argument is `false`.
/// - `some()`: returns `true` if there **is** an inner argument.
/// - `none()`: returns `true` if there is **not** an inner argument.
/// - `all()`: returns `true` if all the arguments are `true`.
/// - `any()`: returns `true` if any argument is `true`.
///
/// # Examples
/// ```
/// # use devela_macros::compile;
/// #[compile(true)]
/// fn compiled() {}
/// #[compile(other_than_true)]
/// fn not_compiled() {}
///
/// // `not()`:
/// #[compile(not(other_than_true))]
/// fn compiled_not() {}
/// #[compile(not(true))]
/// fn not_compiled_not() {}
///
/// // `some()`
/// #[compile(some(thing))]
/// fn compiled_some() {}
/// #[compile(some())]
/// fn not_compiled_some() {}
///
/// // `none()`
/// #[compile(none())]
/// fn compiled_none() {}
/// #[compile(none(thing))]
/// fn not_compiled_none() {}
///
/// // `all()`:
/// #[compile(all(true, true, none(), some(thing), not(none(thing))))]
/// fn compiled_all() {}
/// #[compile(all(true, false))]
/// fn not_compiled_all() {}
///
/// // `any()`:
/// #[compile(any(true, false))]
/// fn compiled_any() {}
/// #[compile(any(false, false))]
/// fn not_compiled_any() {}
///
/// // nested:
/// #[compile(all(true, not(any(some(), none(thing), not(true)))))]
/// fn compiled_nested() {}
/// #[compile(all(true, not(any(some(), none(thing), true))))]
/// fn not_compiled_nested() {}
///
/// compiled();
/// compiled_not();
/// compiled_some();
/// compiled_none();
/// compiled_all();
/// compiled_any();
/// compiled_nested();
///
/// // not_compiled()
/// // not_compiled_not();
/// // not_compiled_some();
/// // not_compiled_none();
/// // not_compiled_all();
/// // not_compiled_any();
/// // not_compiled_nested();
/// ```
#[proc_macro_attribute]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    if eval_arg(argument.to_string()) {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}

#[rustfmt::skip]
fn eval_arg(arg: String) -> bool {
    if arg == "true" {
        true

    } else if arg.starts_with("not(") && arg.ends_with(')') {
        let inner_arg = &arg[4..arg.len() - 1];
        !eval_arg(inner_arg.to_string())

    } else if arg.starts_with("some(") && arg.ends_with(')') {
        let inner_arg = &arg[5..arg.len() - 1];
        !inner_arg.is_empty()

    } else if arg.starts_with("none(") && arg.ends_with(')') {
        let inner_arg = &arg[5..arg.len() - 1];
        inner_arg.is_empty()

    } else if arg.starts_with("all(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        split_args(inner_args).into_iter().all(eval_arg)

    } else if arg.starts_with("any(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        split_args(inner_args).into_iter().any(eval_arg)

    } else {
        false
    }
}

// Argument parser that correctly deals with nested arguments with commas.
fn split_args(arg: &str) -> Vec<String> {
    let mut args = Vec::new();
    let (mut start, mut level) = (0, 0);

    for (i, ch) in arg.chars().enumerate() {
        match ch {
            '(' => level += 1,
            ')' => level -= 1,
            ',' if level == 0 => {
                args.push(arg[start..i].trim().to_string());
                start = i + 1;
            }
            _ => {}
        }
    }

    args.push(arg[start..].trim().to_string());
    args
}
