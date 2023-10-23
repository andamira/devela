// devela::string::ext
//

mod str;
mod string;

pub use str::StrExt;
#[cfg(feature = "alloc")]
pub use string::StringExt;
