#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [package]
//! name = "term_linux"
//! [dependencies.devela]
//! path = "../.."
//! # git = "https://github.com/andamira/devela"
//! features = ["linux", "term", "event"]
//! ```
// devela/examples/sys/term_linux.rs

use devela::*;

fn main() -> LinuxResult<()> {
    let mut term = TermLinux::open()?;
    // let _session = term.session(TermMode::event().mouse_motion_pixels())?;
    let _session = term.session(TermMode::event().mouse_drag_pixels())?;

    term.print(b"TermLinux raw session\r\n")?;
    println!("size: {:?}", term.size());
    println!("term caps: {}", term.term_capabilities());
    println!("run caps: {:?}", term.run_capabilities());
    term.print(b"\r\nPress keys. Ctrl-Q exits.\r\n")?;

    loop {
        if let Some(ev) = term.poll_event()? {
            println!("{ev:?}");
            if let EventKind::Key(k) = ev {
                if k.mods.has_control() && matches!(k.semantic, Key::Char('q') | Key::Q) {
                    break;
                }
            }
        }
    }
    Ok(())
}
