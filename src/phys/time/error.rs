// devela/src/phys/time/error.rs

use crate::{_TAG_TIME, define_error};

define_error! { individual: pub struct Timeout;
    #[derive(Default)],
    +location: "phys/time", +test_size_of(0), +tag: _TAG_TIME!(),
    DOC_TIMEOUT = "The operation has exceeded the allowed execution time.",
    self+f => write!(f, "The operation has exceeded the allowed execution time.")
}
