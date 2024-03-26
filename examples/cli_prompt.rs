#!/usr/bin/env -S rust-script -c
//!
//! A terminal CLI prompt
//!
//! ```cargo
//! [dependencies]
//! devela = { path = "..", features = ["ui_term", "std", "crossterm"] }
//! # <https://crates.io/crates/rust-script> # `cargo install rust-script`
//! ```
//

use devela::ui::term::Prompt;

fn main() {
    println!["hi"];

    let mut placeholder = String::new();

    let name = {
        loop {
            let name = Prompt::new()
                .can_be_empty(false)
                .ask("What is your name?", &placeholder)
                .unwrap();
            let confirm = Prompt::new()
                .can_be_empty(false)
                .max_len(1)
                .validator(|char_idx, _string| {
                    // received a character and its index position (in graphemes)
                    if let Some((c, _idx)) = char_idx {
                        match c {
                            'y' | 'Y' | 'n' | 'N' => (true, None),
                            _ => (false, None),
                        }
                    // if not, we can only validate the final response string
                    } else {
                        (true, None)
                    }
                })
                .ask(&format!["Is your name {name}? Y/N"], "Y")
                .unwrap();
            if matches![confirm.as_ref(), "y" | "Y"] {
                break name;
            } else {
                placeholder = name;
            }
        }
    };

    println!("Hello {name}!");
}
