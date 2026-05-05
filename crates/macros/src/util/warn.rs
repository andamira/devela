// devela_macros::util::warn
//
//! Defines [`warn_tokens()`].
//

#![allow(unused, reason = "depends on nightly_stable_later")]

use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::quote_spanned;

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
                #[warn(deprecated)]
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
