// devela construct::gen
//
//! Code generation during the build process.
//

mod tuple;

pub(super) fn generate() -> Result<(), std::io::Error> {
    tuple::generate()
}
