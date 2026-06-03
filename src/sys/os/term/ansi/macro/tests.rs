// devela::sys::os::term::ansi::macro::tests

use crate::{ansi, const_assert};

#[test]
fn str() {
    assert_eq!["\u{1b}[1m", ansi![s: bold]];
    assert_eq!["\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]];
    assert_eq!["\u{1b}[2;3H", ansi![s: cursor_move1(3, 2)]];

    const_assert![eq_str "\u{1b}[1m", ansi![s: bold]];
    const_assert![eq_str "\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]];
    const_assert![eq_str "\u{1b}[2;3H", ansi![s: cursor_move1(3, 2)]];
}

#[test]
fn bytes() {
    assert_eq![&[27, 91, 49, 109], ansi![b: bold]];
    assert_eq![&[27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]];
    assert_eq![&[27, 91, 50, 59, 51, 72], ansi![b: cursor_MOve1(3, 2)]];

    const_assert![eq_buf & [27, 91, 49, 109], ansi![b: bold]];
    const_assert![eq_buf & [27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]];
    const_assert![eq_buf & [27, 91, 50, 59, 51, 72], ansi![b: cursor_MOve1(3, 2)]];
}

#[test]
#[crate::macro_apply(crate::_std_or_linux_syscall)]
fn print_non_const() {
    fn dyn_args(x: u8, y: u8) {
        let _ = ansi![@p: cursor_save cursor_move1(x, y) cursor_restore];
    }
    dyn_args(3, 5);
    dyn_args(4, 8);
}
