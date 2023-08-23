// devela_macros::common
//
//! Common functionality for procedural macros.
//

#[rustfmt::skip]
pub(crate) fn compile_eval(arg: String) -> bool {
    if arg == "true" {
        true

    } else if arg.starts_with("not(") && arg.ends_with(')') {
        let inner_arg = &arg[4..arg.len() - 1];
        !compile_eval(inner_arg.to_string())

    } else if arg.starts_with("some(") && arg.ends_with(')') {
        let inner_arg = &arg[5..arg.len() - 1];
        !inner_arg.is_empty()

    } else if arg.starts_with("none(") && arg.ends_with(')') {
        let inner_arg = &arg[5..arg.len() - 1];
        inner_arg.is_empty()

    } else if arg.starts_with("all(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        split_args(inner_args).into_iter().all(compile_eval)

    } else if arg.starts_with("any(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        split_args(inner_args).into_iter().any(compile_eval)

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
