// devela::data::list::array::vec::reexports

use crate::{impl_cdef, mod_path};

mod_path!(alloc +pub _a "../../../../../libs/base_alloc/src/data/list/array/vec/reexports.rs");

impl_cdef![ConstDefault: <T> Self::new() => Vec<T>]; // impl ConstDefault
