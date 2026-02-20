//! External sentinel crate used to exercise devela as a downstream dependency.

// #![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
// #![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
// #![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]

extern crate alloc;

// use devela_base_core::buffer_linear;
use devela::buffer_linear;

buffer_linear!(pub struct BufferLinearExample: (u8); array, uninit, option);
buffer_linear!(pub struct BufferLinearViewExample: view (u8); slice_mut, slice);
buffer_linear!(pub struct BufferDynExample: alloc (u8); vec);
