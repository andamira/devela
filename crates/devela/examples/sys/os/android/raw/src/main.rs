//
//!
//

use devela::{Android, AndroidLog, DiagLevel};

fn main() {
    let _android = Android;

    println!("devela android raw");
    println!("os:   {}", std::env::consts::OS);
    println!("arch: {}", std::env::consts::ARCH);
    println!("ptr:  {} bits", usize::BITS);

    let log = AndroidLog::new(c"devela");
    let written = log.write(DiagLevel::Info, c"hello from native devela");

    println!("android log: {}", if written { "written" } else { "filtered" },);
}
