// devela_macros::util
//
//! Utility helpers, some of them copied from `devela`.
//

mod _doc_location; // _doc_location! (COPIED from /src/_doc/_doc.rs)
mod doclink; // doclink! (COPIED from /src/code/util/doclink.rs)
mod warn; // const_warn!

pub(crate) use {_doc_location::*, doclink::*, warn::*};
