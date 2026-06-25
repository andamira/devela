#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [package]
//! name = "term_linux"
//! [dependencies.devela]
//! path = "../../../.."
//! # git = "https://github.com/andamira/devela"
//! features = ["linux", "term", "event"]
//! ```
// devela/examples/sys/os/term/linux.rs

use devela::all::*;

fn main() -> LinuxResult<()> {
    let mut term = TermLinux::open()?;
    term.listen_signals(AppControlSet::Interrupt | AppControlSet::Terminate);

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
            match ev {
                EventKind::Control(AppControl::Interrupt | AppControl::Terminate) => break,
                EventKind::Key(k)
                    if k.mods.has_control() && matches!(k.semantic, Key::Char('q') | Key::Q) =>
                {
                    break;
                }
                _ => (),
            }
        }
    }
    Ok(())
}
