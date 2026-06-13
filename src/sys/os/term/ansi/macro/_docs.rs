// devela/src/sys/os/term/ansi/macro/_docs.rs

crate::CONST! {
    pub(super) _DOC_ANSI = r#"
- the `b:` arm accepts only static command arguments and returns [`&[u8]`](slice).
- the `s:` arm accepts only static command arguments and returns [`&str`].
- the `p:` arm accepts only static command arguments and returns a print result.
- the `p!` arm is the same as `p:`, followed by `.unwrap()`.
- the `@p:` arm accepts dynamic command arguments and returns a print result.
- the `@p!` arm is the same as `@p:`, followed by `.unwrap()`.
- the `renderer=>` arm accepts only static command arguments and pushes them into a renderer.
- the `@renderer=>` arm accepts dynamic command arguments and pushes them into a renderer.

## Printing backends

The print arms use the best available backend for the current build:

1. Linux syscall backend, when available.
2. `std` backend, when `std` is enabled and the Linux syscall backend is not selected.
3. strict fallback backend otherwise.

In fallback mode the print arms do not print,
but they still validate the ANSI commands at compile time.

For example, this still checks that `bold` and `reset` exist:
```ignore
use devela::ansi;
ansi![p: bold, reset].unwrap();
````

The dynamic print arm also validates command names and argument types,
but does not evaluate dynamic arguments in fallback mode.

## Renderer output

The renderer arms push ANSI bytes into the given renderer using `try_push_bytes`.

The static renderer arm concatenates all commands first:

```ignore
ansi![renderer=> reset, cursor_move1(1, 1)]?;
```

The dynamic renderer arm pushes each command in order:

```ignore
ansi![@renderer=> cursor_move1(col, row), colors(fg, bg)]?;
```

The renderer destination should be a cheap receiver expression, such as `r`,
`self.renderer`, or `renderers[i]`. In the dynamic renderer arm the destination
expression is used once per command.

## Notes

- commands are case-insensitive.
- the list of commands can be separated by spaces and/or commas.
- commands without parentheses refer to [`Ansi`][crate::Ansi] associated constants.
- commands with parentheses refer to `Ansi` associated functions.
- only commands that return an array are supported, not `*_N` versions.
- the static arms make use of the [`const_join!`] macro for concatenation.
- the real print backends call the appropriate [`ansi_print`] function variant.
- the fallback print backend returns `Ok(())` with an [`Infallible`] error type.
- the renderer arms call the renderer's `try_push_bytes` method.

# Examples

```
# use devela::{ansi, const_assert};
assert_eq![&[27, 91, 49, 109], ansi![b: bold]];
assert_eq![&[27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]];
assert_eq![&[27, 91, 50, 59, 51, 72], ansi![b: cursor_move1(3, 2)]];
const_assert![eq_buf
&[27, 91, 49, 109, 27, 91, 51, 51, 109, 27, 91, 52, 59, 50, 72, 27, 91, 48, 109],
ansi![b: bold yElLoW, cursor_move1(2, 4) rEsEt]
];

assert_eq!["\u{1b}[1m", ansi![s: bold]];
assert_eq!["\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]];
assert_eq!["\u{1b}[2;3H", ansi![s: cursor_move1(3, 2)]];
const_assert![eq_str
"\u{1b}[1m\u{1b}[33m\u{1b}[4;2H\u{1b}[0m",
ansi![s: bold yElLoW, cursor_move1(2, 4) rEsEt]
];

// Prints when a print backend is available.
// In fallback mode this validates the commands and returns Ok(()).
ansi![p: bold, ITALIC, cursor_move1(3, 2), reset].unwrap();

let row = 2;
let col = 3;

// Prints each command immediately when a print backend is available.
// In fallback mode this validates the commands without evaluating dynamic effects.
ansi![@p: cursor_move1(col, row), reset].unwrap();
```

Renderer examples:
```ignore
use devela::{ansi, Ansi};

fn draw(r: &mut GameRenderer, row: u16, col: u16) -> DrawResult {
    // Equivalent to: r.try_push_bytes(ansi![b: reset])?;
    ansi![r=> reset]?;

    // Equivalent to: r.try_push_bytes(&Ansi::CURSOR_MOVE1_B(col, row))?;
    ansi![@r=> cursor_move1(col, row)]?;

    Ok(())
}
```

[`const_join!`]: crate::const_join
[`ansi_print`]: crate::ansi_print
[`Infallible`]: crate::Infallible

"#;
}
