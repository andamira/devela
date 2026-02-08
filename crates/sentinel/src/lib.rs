//! External sentinel crate used to exercise devela as a downstream dependency.

// #![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
// #![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
// #![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]

use devela_base_core::define_bufline;

define_bufline!(
    pub struct BufLineExample: (u8);
    array,
    uninit,
    option,
    slice_mut, slice,
);
