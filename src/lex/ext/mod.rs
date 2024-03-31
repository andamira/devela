// devela::lex::ext
//

mod slice;
mod string;

pub use slice::ExtStr;
#[cfg(feature = "alloc")]
pub use string::ExtString;
