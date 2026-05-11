// devela_macros::copied
//
//! Copied helpers from `devela`.
//

mod _doc_location; // _doc_location! (COPIED from /src/_doc/_doc.rs)
mod _doc_vendor; // _doc_vendor! (COPIED from /src/_doc/_doc.rs)
mod doclink; // doclink! (COPIED from /src/code/util/doclink.rs)

pub(crate) use {_doc_location::*, _doc_vendor::*, doclink::*};
