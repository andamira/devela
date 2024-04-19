// devela construct
//
//!
//

#[cfg(feature = "__dbg")]
mod dbg; // debugging
mod gen; // code generation

fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    dbg::print_features();

    gen::generate()
}

/* helpers */

#[allow(dead_code)]
pub(crate) fn println(msg: &str) {
    // #[cfg(feature = "__dbg")] // MAYBE
    println!("cargo:warning={}", msg);
}

#[allow(dead_code)]
fn rustfmt_file(file_path: &str) {
    let output = std::process::Command::new("rustfmt")
        .arg(file_path)
        .output()
        .expect("failed to execute rustfmt");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        panic!("rustfmt failed: {}", error_message);
    }
}
