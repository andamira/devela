//
//! Elementary expression and control flow.
//

#[cfg(test)]
mod _test_unwrap;

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
            is::is, lets::lets, punroll::punroll, read_at::read_at, sf::sf, unwrap::unwrap,
            whilst::whilst, write_at::write_at,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::write_at::__unicode_scalar_write_utf8_at;
    }
}
