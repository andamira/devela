crate::mods_in! {
    mod machine;
    mod state;

    #[cfg(feature = "__std")]
    mod trie;
}
