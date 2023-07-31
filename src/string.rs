// devela::string
//
//! String related helpers.
//

#[cfg(feature = "alloc")]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

/// Returns a [`String`] where you always know each character's position.
///
/// A [*counter string*][0] is a graduated string of arbitrary `length`,
/// with a `separator` positioned after the immediately preceding number.
///
/// ## Examples
///
/// ```
/// use devela::string::counter_string;
///
/// assert_eq!("2*4*6*8*11*14*", counter_string(14, '*'));
/// assert_eq!("_3_5_7_9_12_15_", counter_string(15, '_'));
/// ```
///
/// [0]: https://www.satisfice.com/blog/archives/22
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn counter_string(mut length: usize, separator: char) -> String {
    let mut cstr = String::new();

    while length > 0 {
        let mut tmpstr = separator.to_string();
        tmpstr.push_str(&length.to_string().chars().rev().collect::<String>());

        if tmpstr.len() > length {
            tmpstr = tmpstr[..length].to_string();
        }

        cstr.push_str(&tmpstr);
        length -= tmpstr.len();
    }
    cstr.chars().rev().collect::<String>()
}

/// Indents the `input` multi-line string with the given number of `spaces`.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn indent(spaces: usize, string: &str) -> String {
    let indentation = " ".repeat(spaces);

    // Split the string into lines
    let lines: Vec<&str> = string.lines().collect();

    // Add the indentation to each line
    let mut indented_lines: Vec<String> = Vec::new();
    for line in lines {
        indented_lines.push(format!("{}{}", indentation, line));
    }

    // Join the modified lines back together
    indented_lines.join("\n")
}
