// devela::code::util::tests
//
// - https://doc.rust-lang.org/reference/macros-by-example.html

#[test]
/// Tests which non-alphanumeric tokens can prefix macro pattern variables.
// Note: for separating expressions only `;` and `,` are valid.
fn _macro_symbols_pattern() {
    let mut s = 0;
    _macro_symbols!(@s 1);
    _macro_symbols!(#s 1);
    _macro_symbols!(?s 1);
    _macro_symbols!(!s 1);
    _macro_symbols!(:s 1);
    _macro_symbols!(;s 1);
    _macro_symbols!(,s 1);
    _macro_symbols!(.s 1);
    //
    _macro_symbols!(~s 1);
    _macro_symbols!(|s 1);
    _macro_symbols!(^s 1);
    _macro_symbols!(&s 1);
    _macro_symbols!(>s 1);
    _macro_symbols!(<s 1);
    //
    _macro_symbols!(=s 1);
    _macro_symbols!(/s 1);
    _macro_symbols!(+s 1);
    _macro_symbols!(-s 1);
    _macro_symbols!(*s 1);
    _macro_symbols!(%s 1);
    //
    _macro_symbols!(()s 1);
    _macro_symbols!([]s 1);
    _macro_symbols!({}s 1);
    _macro_symbols!("" s 1);
    // don't work:
    // _macro_symbols!($ s 1);
    // _macro_symbols!(' s 1);
    // _macro_symbols!(` s 1);
    // _macro_symbols!(\ s 1);
    assert_eq![s, 24];

    #[rustfmt::skip]
    macro_rules! _macro_symbols {
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
    use _macro_symbols;
}
