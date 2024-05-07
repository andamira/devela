// devela construct::features
//

#[cfg(feature = "__dbg")]
mod dbg;

pub(crate) fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "__dbg")]
    dbg::print_features();

    Ok(())
}
