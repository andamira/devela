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

// splits a tuple of two elements; used for the `compile_doc` macro
#[cfg(feature = "alloc")]
pub(crate) fn split_compile_doc_tuple(tuple: &str) -> (String, String) {
    let tuple = tuple.trim();
    if !tuple.starts_with('(') {
        panic!(
            "Tuple must start with '(', but starts with {:?})",
            tuple.chars().next()
        );
    } else if !tuple.ends_with(')') {
        panic!(
            "Tuple must end with ')', but ends with {:?})",
            tuple.chars().last()
        );
    }

    let mut level = 0;
    let mut in_quotes = false;
    let mut split_index = None;

    for (i, ch) in tuple.chars().enumerate().skip(1) {
        match ch {
            '"' => in_quotes = !in_quotes,
            '(' if !in_quotes => level += 1,
            ')' if !in_quotes => level -= 1,
            ',' if level == 0 && !in_quotes => {
                split_index = Some(i);
                break;
            }
            _ => {}
        }
    }

    let split_index = split_index.expect("Could not find split point in tuple");

    let condition = &tuple[1..split_index].trim();
    let comment = &tuple[split_index + 1..tuple.len() - 1]
        .trim()
        .trim_matches('"');

    if condition.is_empty() || comment.is_empty() {
        panic!("Both condition and comment must be present in the tuple");
    }

    (condition.to_string(), comment.to_string())
}

// de-indents a string
//
// calculates the minimum indentation across all non-empty lines
// and then removes that amount of leading whitespace from each line.
//
// should support spaces and tabs
#[cfg(feature = "alloc")]
pub(crate) fn deindent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start_matches([' ', '\t']).len())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            if line.len() > min_indent {
                &line[min_indent..]
            } else {
                &line[..]
            }
        })
        .map(|line| line.trim_end())
        .collect::<Vec<&str>>()
        .join("\n")
}

// Evaluator of compilation predicates
#[rustfmt::skip] #[cfg(feature = "alloc")]
pub(crate) fn compile_eval(arg: String) -> bool {
    /* unary */

    if arg.is_empty() || arg == "false" {
        false

    } else if arg == "true" {
        true

    } else if arg.starts_with("not(") && arg.ends_with(')') {
        let inner_arg = &arg[4..arg.len() - 1];
        !compile_eval(inner_arg.to_string())

    /* binary */

    } else if arg.starts_with("equal(") && arg.ends_with(')') {
        let inner_args = &arg[6..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && compile_eval(args[0].clone()) == compile_eval(args[1].clone())

    } else if arg.starts_with("xor(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && (compile_eval(args[0].clone()) ^ compile_eval(args[1].clone()))

    /* numeric */

    } else if arg.starts_with("eq(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && args[0].parse::<i128>().ok()
            .zip(args[1].parse::<i128>().ok())
            .filter(|(first_num, second_num)| first_num == second_num)
            .is_some()

    } else if arg.starts_with("ne(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && args[0].parse::<i128>().ok()
            .zip(args[1].parse::<i128>().ok())
            .filter(|(first_num, second_num)| first_num != second_num)
            .is_some()

    } else if arg.starts_with("ge(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && args[0].parse::<i128>().ok()
            .zip(args[1].parse::<i128>().ok())
            .filter(|(first_num, second_num)| first_num >= second_num)
            .is_some()

    } else if arg.starts_with("gt(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && args[0].parse::<i128>().ok()
            .zip(args[1].parse::<i128>().ok())
            .filter(|(first_num, second_num)| first_num > second_num)
            .is_some()

    } else if arg.starts_with("le(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && args[0].parse::<i128>().ok()
            .zip(args[1].parse::<i128>().ok())
            .filter(|(first_num, second_num)| first_num <= second_num)
            .is_some()

    } else if arg.starts_with("lt(") && arg.ends_with(')') {
        let inner_args = &arg[3..arg.len() - 1];
        let args = split_args(inner_args);
        args.len() == 2
            && args[0].parse::<i128>().ok()
            .zip(args[1].parse::<i128>().ok())
            .filter(|(first_num, second_num)| first_num < second_num)
            .is_some()

    /* non-binary */

    } else if arg.starts_with("any(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        split_args(inner_args).into_iter().any(compile_eval)

    } else if arg.starts_with("all(") && arg.ends_with(')') {
        let inner_args = &arg[4..arg.len() - 1];
        split_args(inner_args).into_iter().all(compile_eval)

    } else if arg.starts_with("none(") && arg.ends_with(')') {
        let inner_arg = &arg[5..arg.len() - 1];
        inner_arg.is_empty()

    } else if arg.starts_with("some(") && arg.ends_with(')') {
        let inner_arg = &arg[5..arg.len() - 1];
        !inner_arg.is_empty()

    } else if arg.starts_with("diff(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        let args: Vec<_> = split_args(inner_args);
        args.iter().any(|b| b != &args[0])

    } else if arg.starts_with("same(") && arg.ends_with(')') {
        let inner_args = &arg[5..arg.len() - 1];
        let args: Vec<_> = split_args(inner_args);
        args.iter().all(|b| b == &args[0])

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

    /* similar to configuration options (cfg![], #[cfg()], #[cfg_attr()]) */

    // target_pointer_width
    } else if arg.starts_with("pointer_width_eq(") && arg.ends_with(')') {
        let width_arg = &arg[17..arg.len() - 1];
        width_arg.parse::<u32>().map_or(false, |w| w == usize::BITS)
    } else if arg.starts_with("pointer_width_ne(") && arg.ends_with(')') {
        let width_arg = &arg[18..arg.len() - 1];
        width_arg.parse::<u32>().map_or(false, |w| w != usize::BITS)
    } else if arg.starts_with("pointer_width_ge(") && arg.ends_with(')') {
        let width_arg = &arg[18..arg.len() - 1];
        width_arg.parse::<u32>().map_or(false, |w| w >= usize::BITS)
    } else if arg.starts_with("pointer_width_gt(") && arg.ends_with(')') {
        let width_arg = &arg[18..arg.len() - 1];
        width_arg.parse::<u32>().map_or(false, |w| w > usize::BITS)
    } else if arg.starts_with("pointer_width_le(") && arg.ends_with(')') {
        let width_arg = &arg[18..arg.len() - 1];
        width_arg.parse::<u32>().map_or(false, |w| w <= usize::BITS)
    } else if arg.starts_with("pointer_width_lt(") && arg.ends_with(')') {
        let width_arg = &arg[18..arg.len() - 1];
        width_arg.parse::<u32>().map_or(false, |w| w < usize::BITS)

    // target endian
    } else if arg == "little_endian()" {
        cfg!(target_endian = "little")
    } else if arg == "big_endian()" {
        cfg!(target_endian = "big")

    /* _ */

    } else {
        panic!["Unrecognized compilation predicate: {:?}", arg];
    }
}
