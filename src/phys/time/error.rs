// devela::phys::time::error

use crate::{_TAG_TIME, define_error};

define_error! { individual: pub struct Timeout;
    #[derive(Default)], +location: "phys/time", +tag: _TAG_TIME!(),
    DOC_KEY_ALREADY_EXISTS = "The operation has exceeded the allowed execution time.",
    self+f => write!(f, "The operation has exceeded the allowed execution time.")
}
