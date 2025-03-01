// devela::lang::js::web_api.js

export async function initWasm(wasmPath, imports = {}) {
	let wasmInstance;

    const canvas = document.getElementById("myCanvas");
    if (!canvas) { throw new Error("Canvas element not found"); }
    const ctx = canvas.getContext("2d");
    if (!ctx) { throw new Error("Failed to get 2D rendering context"); }

    const wasmImports = {
        api_canvas: {
			/* Basic Drawing */
            fill_rect: (x, y, w, h) => ctx.fillRect(x, y, w, h),

			/* Color Setting */
            set_color: (r, g, b) => {
                ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
                ctx.strokeStyle = `rgb(${r}, ${g}, ${b})`;
            },

			/* Color Setting */
            draw_line: (x1, y1, x2, y2) => {
                ctx.beginPath();
                ctx.moveTo(x1, y1);
                ctx.lineTo(x2, y2);
                ctx.stroke();
            },
            draw_circle: (x, y, r) => {
                ctx.beginPath();
                ctx.arc(x, y, r, 0, Math.PI * 2);
                ctx.fill();
            },

			/* Text */
            fill_text: (ptr, len, x, y) => {
                ctx.fillText(str_decode(ptr, len), x, y);
            },
        },
        api_console: {
            console_debug: (ptr, len) => console.debug(str_decode(ptr, len)),
            console_info: (ptr, len) => console.info(str_decode(ptr, len)),
            console_log: (ptr, len) => console.log(str_decode(ptr, len)),
            console_warn: (ptr, len) => console.warn(str_decode(ptr, len)),
            console_error: (ptr, len) => console.error(str_decode(ptr, len)),
            console_group: (ptr, len) => console.group(str_decode(ptr, len)),
            console_group_end: () => console.groupEnd(),
        },
        api_timing: {
            get_time: () => performance.now(),
        }
    };

	/* helpers */

	// Decode UTF-8 strings from WASM memory
    function str_decode(ptr, len) {
        const memory = new Uint8Array(wasmInstance.exports.memory.buffer, ptr, len);
        return new TextDecoder("utf-8").decode(memory);
    }

    const finalImports = { ...wasmImports, ...imports };
    const response = await fetch(wasmPath);
    const wasm = await WebAssembly.instantiateStreaming(response, finalImports);

    wasmInstance = wasm.instance;
    wasm.instance.exports.main();

    return wasmInstance;
}
