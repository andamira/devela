//
//! Manual downstream smoke tests for devela.
//!
//! Contains small executable probes used while validating devela
//! from the perspective of an external consumer.
//

// #![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
// #![cfg_attr(all(nightly_doc, miri), allow(unused_attributes))]
// #![cfg_attr(all(nightly_doc, not(doc)), allow(unused_attributes))]

#[allow(unused)]
use devela::unwrap;

fn main() {
    /* test unwrap */

    let _a1 = Some(32);
    let _a2: Option<i32> = None;
    // let _b = unwrap![some _a2];

    // #[cfg(target_os = "linux")]
    // assert_eq![2, 3];
}
