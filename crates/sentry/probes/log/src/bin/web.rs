#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(target_arch = "wasm32")]
use devela::JsConsole;
#[cfg(target_arch = "wasm32")]
use sentry_log::exercise_diag;

#[cfg(target_arch = "wasm32")]
devela::set_panic_handler! { loop }

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let mut out = JsConsole;
    let _ = exercise_diag(&mut out);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
