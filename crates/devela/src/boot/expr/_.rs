//
//! Elementary expression and control flow.
//

#[cfg(test)]
mod _test_unwrap;

mod bitwise;
mod is;
mod lets;
mod punroll;
mod read_at;
mod sf;
mod unwrap;
mod whilst;
mod write_at;

super::mods_out! {
    _mods {
        pub use super::{
            bitwise::bitwise, is::is, lets::lets, punroll::punroll, read_at::read_at, sf::sf,
            unwrap::unwrap, whilst::whilst, write_at::write_at,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            bitwise::{__bitwise, __bitwise_backend, __bitwise_inline, __bitwise_methods},
            write_at::__unicode_scalar_write_utf8_at,
        };
    }
}
