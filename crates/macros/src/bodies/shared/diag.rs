// devela_macros::bodies::shared::diag
//
//! Token-level diagnostics for procedural macro bodies.
//!
//! These helpers emit diagnostics from macro expansion code while keeping stable
//! and nightly behavior close enough for ordinary use.
//!
//! On nightly, native proc-macro diagnostics are emitted directly. On stable,
//! errors expand to `compile_error!`, while warnings use a small deprecated-item
//! fallback so they still appear as compiler warnings.
//!
//! The policy helpers [`warn_tokens`] and [`deny_tokens`] are controlled by:
//! - `--cfg devela_macros_warnings="allow|warn|deny"`
//! - `--cfg devela_macros_errors="warn|deny"`
//!
//! These flags affect only diagnostics intentionally emitted by devela_macros.
//

use proc_macro2::{Span, TokenStream as TokenStream2};
#[cfg(not(nightly_stable_later))]
use {proc_macro2::Literal, quote::quote_spanned};

/// Returns whether macro policy errors are emitted as hard errors.
///
/// When this returns `false`, policy errors are downgraded to warnings through
/// `devela_macros_errors="warn"`.
pub(crate) fn macro_errors_are_denied() -> bool {
    !cfg!(devela_macros_errors = "warn")
}

/// Emits a policy error when `emit` is true.
///
/// By default this emits a hard macro error.
/// If `devela_macros_errors="warn"` is set, the diagnostic is downgraded to a
/// warning. This is useful for macro policies that should normally stop
/// compilation, but may be temporarily relaxed during migration or auditing.
///
/// Returns diagnostic tokens on stable fallbacks,
/// or an empty stream when the diagnostic is emitted directly.
#[must_use]
pub(crate) fn deny_tokens(emit: bool, span: Span, msg: &str) -> TokenStream2 {
    if !emit {
        return TokenStream2::new();
    }
    match macro_error_level() {
        TokenDiag::Warn => warn_tokens_raw(span, msg),
        TokenDiag::Deny => error_tokens(span, msg),
        TokenDiag::Allow => TokenStream2::new(), // unreachable for now
    }
}

/// Emits an unconditional macro error.
///
/// On nightly, this uses native proc-macro diagnostics.
/// On stable, it emits a spanned `compile_error!`.
///
/// This is not affected by `devela_macros_errors`;
/// use [`deny_tokens`] for policy errors that may be downgraded.
#[must_use]
pub(crate) fn error_tokens(_span: Span, msg: &str) -> TokenStream2 {
    cfg_select! {
        nightly_stable_later => {
            proc_macro::Span::call_site().error(msg).emit();
            TokenStream2::new()
        },
        _ => {
            let msg = Literal::string(msg);
            quote_spanned! { _span =>
                compile_error!(#msg);
            }
        }
    }
}

/// Emits a policy warning when `emit` is true.
///
/// By default this emits a warning.
/// If `devela_macros_warnings="allow"` is set, the diagnostic is suppressed.
/// If `devela_macros_warnings="deny"` is set, it is promoted to a hard macro error.
///
/// Returns diagnostic tokens on stable fallbacks,
/// or an empty stream when the diagnostic is emitted directly.
#[must_use]
pub(crate) fn warn_tokens(emit: bool, span: Span, msg: &str) -> TokenStream2 {
    if !emit {
        return TokenStream2::new();
    }
    match macro_warning_level() {
        TokenDiag::Allow => TokenStream2::new(),
        TokenDiag::Warn => warn_tokens_raw(span, msg),
        TokenDiag::Deny => error_tokens(span, msg),
    }
}

/* internal helpers */

/// Emits a raw macro warning without applying devela_macros warning policy.
///
/// On nightly, this uses `proc_macro::Span::warning`.
/// On stable, it expands to a tiny deprecated-item use,
/// producing a warning with `msg` in the deprecation note.
///
/// Most macro bodies should call [`warn_tokens`] instead,
/// so user policy flags are respected.
#[must_use]
pub fn warn_tokens_raw(_span: Span, msg: &str) -> TokenStream2 {
    cfg_select! {
        nightly_stable_later => {
            proc_macro::Span::call_site().warning(msg).emit();
            TokenStream2::new()
        },
        _ => {
            let msg = Literal::string(&format!["\n{msg:?}"]);
            quote_spanned! { _span =>
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

/// Effective diagnostic level selected by devela_macros cfg policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenDiag {
    /// Suppress the diagnostic.
    Allow,
    /// Emit the diagnostic as a warning.
    Warn,
    /// Emit the diagnostic as an error.
    Deny,
}
/// Returns the configured policy for macro warnings.
///
/// Defaults to [`TokenDiag::Warn`].
fn macro_warning_level() -> TokenDiag {
    if cfg!(devela_macros_warnings = "allow") {
        TokenDiag::Allow
    } else if cfg!(devela_macros_warnings = "deny") {
        TokenDiag::Deny
    } else {
        TokenDiag::Warn
    }
}
/// Returns the configured policy for macro errors.
///
/// Defaults to [`TokenDiag::Deny`].
fn macro_error_level() -> TokenDiag {
    if cfg!(devela_macros_errors = "warn") {
        TokenDiag::Warn
    } else {
        TokenDiag::Deny
    }
}
