// devela_base::phys::time::errors
//
//! Time-related errors.
//

use crate::define_error;

/* individual errors */

define_error! { individual: pub struct Timeout;
    DOC_KEY_ALREADY_EXISTS = "The operation has exceeded the allowed execution time.",
    self+f => write!(f, "The operation has exceeded the allowed execution time.")
}
