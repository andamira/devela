//
//! Downstream validation of devela's public API and behavior.
//!
//! Exercises devela as an external consumer to catch regressions and validate
//! assumptions that cannot be tested faithfully from inside the main crate.
//!
//! This crate intentionally remains outside the devela workspace.
//

// #![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
// #![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
// #![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]

extern crate alloc;

mod all_imports {
    use devela::all::*;

    #[allow(dead_code)]
    #[macro_derive(Debug)]
    pub struct Test;
}

pub mod buffer_linear {
    use devela::buffer_linear;

    buffer_linear!(pub struct BufferLinearExample: (u8); array, uninit, option);
    buffer_linear!(pub struct BufferLinearViewExample: view (u8); slice_mut, slice);
    buffer_linear!(pub struct BufferDynExample: alloc (u8); vec);
}

pub mod rand_pcg {
    devela::rand_pcg![pub Pcg16: (u16)];
}
