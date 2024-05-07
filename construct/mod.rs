// devela construct
//
//! The build script.
//

mod features;
mod generate;

fn main() -> Result<(), std::io::Error> {
    features::main()?;
    generate::main()?;

    Ok(())
}

/* helpers */

#[allow(dead_code)]
pub(crate) fn println(msg: &str) {
    // #[cfg(feature = "__dbg")] // MAYBE
    println!("cargo:warning={}", msg);
}

// FIXME for CI
#[allow(dead_code)]
fn rustfmt_file(file_path: &str) {
    use std::process::Command;

    // for now, only call rustfmt if we can find it, on unix systems
    if match Command::new("which").arg("rustfmt").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    } {
        let output = Command::new("rustfmt")
            .arg(file_path)
            .output()
            .expect("failed to execute rustfmt");
        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            panic!("rustfmt failed: {}", error_message);
        }
    }
}
