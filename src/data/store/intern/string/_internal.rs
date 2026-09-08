// devela/src/data/store/intern/string/_internal.rs
//
//! Defines [`__intern_string!`].
//

#[doc(hidden)]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! __intern_string· {
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__intern_string! {
            %normalize_cursor
            [kind: $($kind)?]
            [index: $iprim + $iprim]
            $($rest)*
        }
    };
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        $($rest:tt)*
    ) => {
        $crate::__intern_string! {
            %normalize_cursor
            [kind: $($kind)?]
            [index: $iprim + $Index]
            $($rest)*
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__intern_string! {
            %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [cursor: $cprim + $cprim]
            $($rest)*
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        $($rest:tt)*
    ) => {
        $crate::__intern_string! {
            %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [cursor: $cprim + $Cursor]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        [interner: $(#[$interner_attr:meta])* $vis:vis $Interner:ident]
        [symbol: $(#[$symbol_attr:meta])* $svis:vis $Symbol:ident]
    ) => {
        $crate::handle! {
            [index: $iprim + $Index;]

            $(#[$symbol_attr])*
            $svis $Symbol;
        }
        $crate::__intern_string! {
            %backend
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [cursor: $cprim + $Cursor]
            [interner: $(#[$interner_attr])* $vis $Interner]
            [symbol: $svis $Symbol]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*
    ) => {
        $crate::__intern_string! {
            %backend
            [kind: static]
            $($rest)*
        }
    };
    (%backend
        [kind: static]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        [interner: $(#[$interner_attr:meta])* $vis:vis $Interner:ident]
        [symbol: $svis:vis $Symbol:ident]
    ) => {
        $crate::paste! {
            $crate::__intern_string_impl_array! {
                [index: $iprim + $Index;]
                [cursor: $cprim + $Cursor;]
                [arena: [<__ $Interner StringArena>];]

                $(#[$interner_attr])*
                $vis $Interner;

                $svis $Symbol;
            }
        }
    };
    (%backend
        [kind: alloc]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        [interner: $(#[$interner_attr:meta])* $vis:vis $Interner:ident]
        [symbol: $svis:vis $Symbol:ident]
    ) => {
        $crate::paste! {
            $crate::__intern_string_impl_vec! {
                [index: $iprim + $Index;]
                [cursor: $cprim + $Cursor;]
                [arena: [<__ $Interner StringArena>];]

                $(#[$interner_attr])*
                $vis $Interner;

                $svis $Symbol;
            }
        }
    };
}
pub use __intern_string· as __intern_string;
