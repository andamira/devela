// devela_macros::bodies::compile
//
//! Bodies related to compile control.
//
// TOC
// - cif
// - compile
// - compile_attr
// - compile_doc

use super::shared::{compile_eval, deindent, split_args, split_compile_doc_tuple};
use ::core::fmt::Write;
use proc_macro::TokenStream;

#[inline(always)]
pub(crate) fn body_cif(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let result = compile_eval(input);
    result.to_string().parse().unwrap()
}

#[inline(always)]
pub(crate) fn body_compile(args: TokenStream, input: TokenStream) -> TokenStream {
    if compile_eval(args.to_string()) { input } else { TokenStream::new() }
}

#[inline(always)]
pub(crate) fn body_compile_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let mut args = split_args(&args);
    if args.is_empty() {
        panic!("The compile_attr macro requires at least one argument");
    }
    let condition = args.remove(0);

    if compile_eval(condition) {
        let mut expanded = String::new();
        for attr in &args {
            write!(&mut expanded, "#[{}] ", attr).unwrap();
        }
        expanded.push_str(&input.to_string());
        expanded.parse().expect("Couldn't parse as a TokenStream")
    } else {
        input
    }
}

#[inline(always)]
pub(crate) fn body_compile_doc(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let doc_conditions = split_args(&args);
    let mut result = String::new();

    for doc_condition in doc_conditions {
        if doc_condition.is_empty() {
            break;
        }
        let (condition, comment) = split_compile_doc_tuple(&doc_condition);
        if compile_eval(condition) {
            write!(&mut result, "#[doc = \"{}\n\"]", deindent(&comment)).unwrap();
        }
    }
    // Append the original item
    result.push_str(&input.to_string());
    result.parse().unwrap()
}
