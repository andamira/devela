// devela_macros/tests/mods.rs
//
//!
//

use devela_macros::mods_in;

mods_in! {
    pub mod mods_leaf;
    pub(crate) mod_ mods_branch;
    mod r#mods_raw_leaf;
    pub mod_ r#mods_raw_branch;

    // Also verifies that attributes survive expansion:
    // this file deliberately does not exist.
    #[cfg(any())]
    mod definitely_missing;
}

#[test]
fn resolves_all_module_layouts() {
    assert_eq!(mods_leaf::VALUE, 1);
    assert_eq!(mods_branch::VALUE, 2);
    assert_eq!(r#mods_raw_leaf::VALUE, 3);
    assert_eq!(r#mods_raw_branch::VALUE, 4);
}
