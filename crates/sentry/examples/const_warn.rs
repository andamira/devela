use devela::const_warn;
fn main() {
    const_warn!("custom warning message");
    const_warn!(if size_of::<u8>() == 1, "a byte is just one byte");
}
