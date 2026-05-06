// devela_macros::util::warn
//
//! Defines [`warn_tokens()`].
//

#![allow(unused, reason = "depends on nightly_stable_later")]

use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::quote_spanned;

/// Emits a procedural macro error when `emit` is true.
///
/// On nightly, this uses native proc-macro diagnostics at the macro
/// invocation site. On stable, it emits a spanned `compile_error!`.
///
/// Unlike [`warn_tokens`], the stable fallback is direct and does not need
/// a lint-hijacking trick.
#[must_use]
pub(crate) fn error_tokens(emit: bool, span: Span, msg: &str) -> TokenStream2 {
    if !emit {
        return TokenStream2::new();
    }
    cfg_select! {
        nightly_stable_later => {
            proc_macro::Span::call_site().error(msg).emit();
            TokenStream2::new()
        },
        _ => {
            let msg = Literal::string(msg);
            quote_spanned! { span =>
                compile_error!(#msg);
            }
        }
    }
}

/// Emits a compile-time warning from a procedural macro.
///
/// On nightly, this uses `proc_macro::Span::warning`,
/// producing a direct diagnostic at the macro invocation site.
///
/// On stable, this falls back to generating a tiny deprecated-item use.
/// The fallback warning is less pretty, but carries `msg` as a real
/// diagnostic note instead of encoding it into an unused identifier.
///
/// Returns tokens only for the stable fallback.
/// Nightly emits immediately and returns an empty stream.
#[must_use]
pub(crate) fn warn_tokens(emit: bool, span: Span, msg: &str) -> TokenStream2 {
    if !emit {
        return TokenStream2::new();
    }
    cfg_select! {
        nightly_stable_later => {
            proc_macro::Span::call_site().warning(msg).emit();
            TokenStream2::new()
        },
        _ => {
            let msg = Literal::string(&format!["\n{msg:?}"]);
            quote_spanned! { span =>
                #[allow(dead_code)]
                // #[warn(deprecated)] // make it allowable
                const _: () = {
                    struct __;
                    impl __ {
                        #[deprecated(note = #msg)]
                        const __WARNING: () = ();
                    }
                    let _ = __::__WARNING;
                };
            }
        }
    }
}
