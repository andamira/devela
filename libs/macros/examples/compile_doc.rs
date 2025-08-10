#![allow(dead_code)]

use devela_macros::compile_doc;

#[compile_doc(
    (true, "# Title\n"),
    (true, "- 1st list element."),
    (none(some), "- this wont be compiled."),
    (any(true, false), "- 2nd list element.\n"),

    // Be mindful of indentation
    (same("a", "a"), "First paragraph.

Second paragraph.

    // This is a code block

Third paragraph.")
)]
struct A;

fn main() {}
