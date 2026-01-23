// devela_macros::bodies::repeat
//
//! Body of [`repeat!`] and required functions.
//
// TOC
// * repeat
// - generate_repeat_code
// - parse_repeat_input
// - evaluate_expression
// - find_operator
// - find_outer_parentheses

use ::core::fmt::Write;
use proc_macro::TokenStream;

#[inline(always)]
pub(crate) fn body_repeat(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim();

    let (count_str, expr) = parse_repeat_input(input).expect(
        "Invalid repeat! syntax. Expected: repeat!(COUNT, EXPRESSION)\n\
         Example: repeat!(5, println!(\"hello\"))",
    );
    let count = evaluate_expression(count_str)
        .unwrap_or_else(|err| panic!("Invalid count expression '{}': {:?}", count_str, err));
    if count < 0 {
        panic!("Count cannot be negative: {}", count);
    }
    generate_repeat_code(count as usize, expr)
}

// Generate: expr; expr; expr; ... (count times)
fn generate_repeat_code(count: usize, expr: &str) -> TokenStream {
    if count == 0 {
        return "".parse().unwrap();
    }
    let mut s = String::new();
    s.reserve((expr.len() + 2) * count);
    for _ in 0..count {
        write!(&mut s, "{}; ", expr).unwrap();
    }
    s.parse().unwrap_or_else(|_| {
        panic!(
            "Failed to parse generated code. Expression '{}' may contain invalid Rust syntax",
            expr
        )
    })
}

// Find the first comma that's not inside parentheses/braces
fn parse_repeat_input(input: &str) -> Option<(&str, &str)> {
    let mut depth = 0;
    let mut comma_pos = None;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' | '{' | '[' => depth += 1,
            ')' | '}' | ']' => depth -= 1,
            ',' if depth == 0 => {
                comma_pos = Some(i);
                break;
            }
            _ => {}
        }
    }
    let comma_pos = comma_pos?;
    let count_str = input[..comma_pos].trim();
    let expr = input[comma_pos + 1..].trim();
    Some((count_str, expr))
}

// Simple arithmetic evaluator (supports +, -, *, / with integers)
fn evaluate_expression(expr: &str) -> Result<i64, ()> {
    let expr = expr.trim();
    // Handle parentheses first
    if let Some((start, end)) = find_outer_parentheses(expr) {
        let inner = &expr[start + 1..end];
        let inner_result = evaluate_expression(inner)?;
        let new_expr = format!("{}{}{}", &expr[..start], inner_result, &expr[end + 1..]);
        return evaluate_expression(&new_expr);
    }
    // Handle operators in order of precedence
    for &op in &['+', '-', '*', '/'] {
        if let Some(pos) = find_operator(expr, op) {
            let left = evaluate_expression(&expr[..pos])?;
            let right = evaluate_expression(&expr[pos + 1..])?;
            return match op {
                '+' => Ok(left + right),
                '-' => Ok(left - right),
                '*' => Ok(left * right),
                '/' => {
                    if right == 0 {
                        Err(())
                    } else {
                        Ok(left / right)
                    }
                }
                _ => unreachable!(),
            };
        }
    }
    // Base case: just a number
    expr.parse().map_err(|_| ())
}

fn find_operator(expr: &str, op: char) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in expr.chars().enumerate() {
        match c {
            '(' | '{' | '[' => depth += 1,
            ')' | '}' | ']' => depth -= 1,
            _ => {}
        }
        if depth == 0 && c == op {
            return Some(i);
        }
    }
    None
}

fn find_outer_parentheses(expr: &str) -> Option<(usize, usize)> {
    if expr.starts_with('(') {
        let mut depth = 1;
        for (i, c) in expr.chars().enumerate().skip(1) {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((0, i));
                    }
                }
                _ => {}
            }
        }
    }
    None
}
