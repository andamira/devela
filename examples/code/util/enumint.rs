// devela/examples/code/util/enumint.rs

use devela::enumint;

enumint![MyEnumint, i8, -126, 126];

fn main() {
    assert_eq!(MyEnumint::VALUES, 253);
    assert_eq!(MyEnumint::NICHES, 3);
}
