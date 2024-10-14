// examples/linux.rs

// #!/usr/bin/env rust-script
// //! ```cargo
// //! [dependencies]
// //! devela = { path = "..", features = ["linux", "unsafe_syscall"] }
// //! ```

use devela::all::*;

fn main() {
    /* terminal */

    linux_println("New linux terminal in raw mode...");
    let term = LinuxTerminal::new_raw().expect("linux terminal");

    println!("is_terminal: {}", term.is_terminal());
    println!("terminal size: {:?}", term.size());

    /* read */

    linux_println("\nGet 3 bytes.");
    let mut counter = 0;
    while counter < 3 {
        let b = linux_get_byte();
        println!("byte {counter} = {b} ({})", char::from(b));
        counter += 1;
    }

    linux_println("\nGet 3 chars.");
    let mut counter = 0;
    while counter < 3 {
        let c = linux_get_char();
        println!("char {counter} = {c:?}");
        counter += 1;
    }

    linux_println("\nPause until pressing any of: ('y', '€').");
    linux_pause_until_char(&['y', '€']);

    /* write */

    linux_print("\nthis is ");
    linux_println("an stdout writing test.");

    linux_eprint("and this is ");
    linux_eprintln("an stderr writing test.");

    linux_eprint("and these are bytes: ");
    linux_print_bytes(&[60, 61, 62, 107, 108, 109, 80, 81, 82, b'\n']);

    /* rand */

    println!("\nrandom u16: {}", linux_random_u16());
}
