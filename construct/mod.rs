// devela build script
//
//!
//

#[cfg(feature = "__dbg")]
mod dbg;

fn main() {
    #[cfg(feature = "__dbg")]
    dbg::print_features();
}
