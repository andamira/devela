//
//!
//

use devela::Android;

fn main() {
    let _android = Android;
    println!("devela android raw");
    println!("os:   {}", std::env::consts::OS);
    println!("arch: {}", std::env::consts::ARCH);
    println!("ptr:  {} bits", usize::BITS);
}
