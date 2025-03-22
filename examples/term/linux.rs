#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! devela = { path = "../..", features = ["linux", "unsafe_syscall"] }
//! ```
// devela::examples::term::linux

use devela::{Linux, LinuxResult};

fn main() -> LinuxResult<()> {
    /* terminal */

    Linux::println("New linux terminal in raw mode...")?;
    let _term_guard = Linux::scoped_raw_mode().unwrap();

    println!("is_terminal: {}", Linux::is_terminal());
    println!("terminal size: {:?}", Linux::terminal_size()?);

    /* read */

    Linux::println("\nGet 3 bytes.")?;
    let mut counter = 0;
    while counter < 3 {
        let b = Linux::get_byte()?;
        println!("byte {counter} = {b} ({})", char::from(b));
        counter += 1;
    }

    Linux::println("\nGet 3 chars.")?;
    let mut counter = 0;
    while counter < 3 {
        let c = Linux::get_char()?;
        println!("char {counter} = {c:?}");
        counter += 1;
    }

    Linux::println("\nPause until pressing any of: ('y', '€').")?;
    Linux::pause_until_char(&['y', '€']);

    /* write */

    Linux::print("\nthis is ")?;
    Linux::println("an stdout writing test.")?;

    Linux::eprint("and this is ")?;
    Linux::eprintln("an stderr writing test.")?;

    Linux::eprint("and these are bytes: ")?;
    Linux::print_bytes(&[60, 61, 62, 107, 108, 109, 80, 81, 82, b'\n'])?;

    /* rand */

    println!("\nrandom u16: {}", Linux::random_u16()?);
    Ok(())
}
