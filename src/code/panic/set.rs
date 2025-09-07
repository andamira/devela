// devela::code::panic::set
//
//! Defines the [`set_panic_handler!`] macro.
//

/// Sets a panic handler based on the chosen strategy.
///
/// - `loop`: Enters an infinite loop, ensuring the program halts without undefined behavior.
/// - `unreachable`: optimally halts execution based on the target architecture.
///   - `wasm32`: Uses `unreachable()` to signal an unrecoverable state.
///   - `x86_64`: Uses `_mm_pause()` to reduce CPU power consumption.
///   - `aarch64`: Uses `wfe` (wait-for-event) as a minimal halt.
///   - `riscv64`: Uses `wfi` (wait-for-interrupt) to idle the core.
///   - **Fallback:** Uses an infinite loop.
///   - (It uses `unsafe`, except for `wasm32` and the fallback.
/// - `web`: Logs panic info to the Web console. It requires the `js` feature.
///   - Accepts the size of the log buffer size in bytes. Defaults to `1024` bytes.
/// - `custom`: Uses a user-provided function (returning -> !) as the panic handler.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! set_panic_handler {
    (loop) => {
        #[panic_handler]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            ::core::hint::spin_loop();
            loop {}
        }
    };
    (unreachable) => {
        #[panic_handler]
        #[allow(unreachable_code, reason = "alternative architectures")]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            // Signals an unrecoverable state to the WebAssembly runtime
            // https://doc.rust-lang.org/core/arch/wasm32/fn.unreachable.html
            #[cfg(target_arch = "wasm32")]
            ::core::arch::wasm32::unreachable();

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            unsafe {
                // Low-power pause instruction, avoids busy loops
                // https://doc.rust-lang.org/core/arch/x86_64/fn._mm_pause.html
                // ::core::arch::x86_64::_mm_pause(); // (less efficient than next ones)

                $crate::asm!("pause"); // Reduce power in spin loops
                $crate::asm!("hlt", options(nomem, nostack)); // Try to halt first (low power)
                $crate::asm!("ud2", options(nomem, nostack)); // Force crash if wfi fails
            }

            #[cfg(target_arch = "arm")]
            unsafe {
                // MAYBE Debugger breakpoint
                // #[cfg(debug_assertions)]
                // $crate::asm!("bkpt #0", options(nomem, nostack));

                $crate::asm!("wfi"); // Low-power sleep (best-effort)
                $crate::asm!("udf #0"); // Force crash if wfi fails
            }

            #[cfg(target_arch = "aarch64")]
            unsafe {
                // No-op (alternative could be `wfe` for efficiency)
                // https://doc.rust-lang.org/core/arch/aarch64/fn.__nop.html
                // https://doc.rust-lang.org/core/arch/aarch64/fn.__wfe.html (experimental)
                // ::core::arch::aarch64::__nop();

                // WAIT: https://github.com/rust-lang/rust/issues/117218
                // ::core::arch::aarch64::__wfe(); // better power efficiency
                $crate::asm!("wfe"); // Low-power sleep (better for multi-core than wfi)
                $crate::asm!("udf #0"); // Force crash
            }

            #[cfg(any_target_riscv)]
            unsafe {
                $crate::asm!("wfi"); // Low-power sleep
                $crate::asm!("ebreak"); // Debugger trap
                $crate::asm!("unimp"); // Hard guarantee
            }

            ::core::hint::spin_loop();
            loop {} // Always fallback to avoid UB if nothing else applies
        }
    };
    (web) => {
        $crate::set_panic_handler!(web: 1024);
    };
    (web: $buffer_bytes:literal) => {
        #[panic_handler]
        fn panic(info: &::core::panic::PanicInfo) -> ! {
            #[cfg(target_arch = "wasm32")]
            {
                let mut buf = [0u8; $buffer_bytes];

                // Extract and log the panic message
                $crate::JsConsole::group("#[panic_handler]");
                match $crate::format_buf![&mut buf, "{}", info.message()] {
                    Ok(msg_str) => $crate::JsConsole::debug(msg_str),
                    Err(truncated) => {
                        $crate::JsConsole::debug(truncated);
                        $crate::JsConsole::warn("Panic message was truncated!");
                    }
                }

                // Extract and log the panic location
                match info.location()
                    .map(|loc| $crate::format_buf![&mut buf, "At {}:{}:{}",
                        loc.file(), loc.line(), loc.column()])
                    .unwrap_or(Ok("<panic location unknown>".into()))
                {
                    Ok(loc_str) => $crate::JsConsole::debug(loc_str),
                    Err(truncated) => {
                        $crate::JsConsole::debug(truncated);
                        $crate::JsConsole::warn("Panic location was truncated!");
                    }
                }
                $crate::JsConsole::group_end();

                ::core::arch::wasm32::unreachable();
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                ::core::hint::spin_loop();
                loop {}
            }
            // #[cfg(not(target_arch = "wasm32"))] // MAYBE
            // compile_error!("`web` strategy is only supported on `wasm32` targets.");
        }
    };
    (custom, $func:path) => {
        #[panic_handler]
        fn panic(info: &::core::panic::PanicInfo) -> ! {
            $func(info)
        }
    };
}
#[doc(inline)]
pub use set_panic_handler;
