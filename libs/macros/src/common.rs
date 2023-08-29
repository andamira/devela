// devela_macros::common
//
//! Common functionality for procedural macros.
//

#[cfg(feature = "alloc")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

// Argument parser that correctly deals with nested arguments with commas.
#[cfg(feature = "alloc")]
pub(crate) fn split_args(arg: &str) -> Vec<String> {
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

// Evaluator of compilation predicates
#[rustfmt::skip]
#[cfg(feature = "alloc")]
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

    } else if arg.starts_with("eq(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && compile_eval(args[0].clone()) == compile_eval(args[1].clone())

    } else if arg.starts_with("ne(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && compile_eval(args[0].clone()) != compile_eval(args[1].clone())

    } else if arg.starts_with("same(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        let args: Vec<_> = split_args(inner_args);
        args.iter().all(|b| b == &args[0])

    } else if arg.starts_with("diff(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        let args: Vec<_> = split_args(inner_args);
        args.iter().any(|b| b != &args[0])

    } else if arg.starts_with("xor(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && (compile_eval(args[0].clone()) ^ compile_eval(args[1].clone()))

    // generalization of xor that emphasizes the limited inclusivity property
    } else if arg.starts_with("xany(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        let args = split_args(inner_args);
        let trues = args.iter()
            .map(|x| compile_eval(x.clone())).filter(|&b| b).count();
        trues > 0 && trues < args.len()

    // common generalization of xor that emphasizes the oddness property
    } else if arg.starts_with("xodd(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        split_args(inner_args).into_iter()
            .map(compile_eval).filter(|&b| b).count() % 2 == 1

    // generalization of xor that emphasizes the singular inclusivity property
    } else if arg.starts_with("xone(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        let args = split_args(inner_args);
        let trues = args.iter()
            .map(|x| compile_eval(x.clone())).filter(|&b| b).count();
        trues == 1

    } else {
        false
    }
}
