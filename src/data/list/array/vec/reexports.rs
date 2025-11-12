// devela::data::list::array::vec::reexports

use crate::{_impl_init, mod_path};

mod_path!(alloc +pub _a "../../../../../libs/base_alloc/src/data/list/array/vec/reexports.rs");

_impl_init![ConstInit: <T> Self::new() => Vec<T>];
