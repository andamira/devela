// devela::lang::js::web_api.js
//
// In sync with `./web_api.js`.

export async function initWasm(wasmPath, imports = {}) {
	/* config */

	let wasmInstance;
	let canvas = null;
	let ctx = null;

	/* helpers */

	// Decode UTF-8 strings from WASM memory
	function str_decode(ptr, len) {
		const memory = new Uint8Array(wasmInstance.exports.memory.buffer, ptr, len);
		return new TextDecoder("utf-8").decode(memory);
	}

	// Sets the active canvas by id.
	function set_canvas(canvasId) {
		const newCanvas = document.getElementById(canvasId);
		if (!newCanvas) {
			console.error(`Canvas with ID "${canvasId}" not found`);
			return;
		}
		const newCtx = newCanvas.getContext("2d");
		if (!newCtx) {
			console.error(`Failed to get 2D context for canvas "${canvasId}"`);
			return;
		}
		canvas = newCanvas;
		ctx = newCtx;
	}

	/* bindings */

	const wasmBindings = {
		api_canvas: {
			/* custom */
			set_canvas: (ptr, len) => { set_canvas(str_decode(ptr, len)); },

			/* basic drawing */
			fill_rect: (x, y, w, h) => ctx.fillRect(x, y, w, h),

			/* color Setting */
			set_color: (r, g, b) => {
				ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
				ctx.strokeStyle = `rgb(${r}, ${g}, ${b})`;
			},

			/* shapes */
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

			/* text */
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

	const finalImports = { ...wasmBindings, ...imports };
	const response = await fetch(wasmPath);
	const wasm = await WebAssembly.instantiateStreaming(response, finalImports);

	wasmInstance = wasm.instance;
	wasm.instance.exports.main();

	return wasmInstance;
}
