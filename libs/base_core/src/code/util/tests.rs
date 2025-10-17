// devela_base_core::code::util::tests
//
// - https://doc.rust-lang.org/reference/macros-by-example.html

#[test]
/// Tests which non-alphanumeric tokens can prefix macro pattern variables.
fn macro_symbols() {
    let mut s = 0;
    macro_symbols!(@s 1);
    macro_symbols!(#s 1);
    macro_symbols!(?s 1);
    macro_symbols!(!s 1);
    macro_symbols!(:s 1);
    macro_symbols!(;s 1);
    macro_symbols!(,s 1);
    macro_symbols!(.s 1);
    //
    macro_symbols!(~s 1);
    macro_symbols!(|s 1);
    macro_symbols!(^s 1);
    macro_symbols!(&s 1);
    macro_symbols!(>s 1);
    macro_symbols!(<s 1);
    //
    macro_symbols!(=s 1);
    macro_symbols!(/s 1);
    macro_symbols!(+s 1);
    macro_symbols!(-s 1);
    macro_symbols!(*s 1);
    macro_symbols!(%s 1);
    //
    macro_symbols!(()s 1);
    macro_symbols!([]s 1);
    macro_symbols!({}s 1);
    macro_symbols!("" s 1);
    // don't work:
    // macro_symbols!($ s 1);
    // macro_symbols!(' s 1);
    // macro_symbols!(` s 1);
    // macro_symbols!(\ s 1);
    assert_eq![s, 24];

    #[rustfmt::skip]
    macro_rules! macro_symbols {
        (@$id:ident $val:expr) => { $id += $val; };
        (#$id:ident $val:expr) => { $id += $val; };
        (?$id:ident $val:expr) => { $id += $val; };
        (!$id:ident $val:expr) => { $id += $val; };
        (:$id:ident $val:expr) => { $id += $val; };
        (;$id:ident $val:expr) => { $id += $val; };
        (,$id:ident $val:expr) => { $id += $val; };
        (.$id:ident $val:expr) => { $id += $val; };
        //
        (~$id:ident $val:expr) => { $id += $val; };
        (|$id:ident $val:expr) => { $id += $val; };
        (^$id:ident $val:expr) => { $id += $val; };
        (&$id:ident $val:expr) => { $id += $val; };
        (>$id:ident $val:expr) => { $id += $val; };
        (<$id:ident $val:expr) => { $id += $val; };
        //
        (=$id:ident $val:expr) => { $id += $val; };
        (/$id:ident $val:expr) => { $id += $val; };
        (+$id:ident $val:expr) => { $id += $val; };
        (-$id:ident $val:expr) => { $id += $val; };
        (*$id:ident $val:expr) => { $id += $val; };
        (%$id:ident $val:expr) => { $id += $val; };
        //
        (()$id:ident $val:expr) => { $id += $val; };
        ([]$id:ident $val:expr) => { $id += $val; };
        ({}$id:ident $val:expr) => { $id += $val; };
        (""$id:ident $val:expr) => { $id += $val; };
        // don't work:
        // ($ $id:ident $val:expr) => { $id += $val; };
        // (''$id:ident $val:expr) => { $id += $val; };
        // (`$id:ident $val:expr) => { $id += $val; };
        // (\$id:ident $val:expr) => { $id += $val; };
    }
    use macro_symbols;
}
