// devela::text::ext
//

mod str;
mod string;

pub use str::ExtStr;
#[cfg(feature = "alloc")]
pub use string::ExtString;
