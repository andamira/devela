// devela/src/data/topol/ord/seq/_.rs
//
//! Ordered sequence topology and succession.
//

crate::mods_in! {
    mod node; // Recursive sequence composition
    mod prev_next; // Local succession
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            node::SeqNode,
            prev_next::{SeqNext, SeqPrevNext},
        };
    }
}
