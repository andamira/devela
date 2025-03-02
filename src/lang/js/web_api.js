// devela::lang::js::web_api.js
//
// In sync with `./web_api.rs`.

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

		/* core APIs */

		api_console: {
			// Console API
			console_debug: (ptr, len) => console.debug(str_decode(ptr, len)),
			console_error: (ptr, len) => console.error(str_decode(ptr, len)),
			console_info: (ptr, len) => console.info(str_decode(ptr, len)),
			console_log: (ptr, len) => console.log(str_decode(ptr, len)),
			console_trace: () => console.trace(),
			console_warn: (ptr, len) => console.warn(str_decode(ptr, len)),
			//
			console_group: (ptr, len) => console.group(str_decode(ptr, len)),
			console_groupEnd: () => console.groupEnd(),
		},
		api_events: {
			_callbacks: new Map(),
			event_addListener: (elementPtr, elementLen, eventPtr, eventLen, callbackPtr) => {
				const element = document.querySelector(str_decode(elementPtr, elementLen));
				const event = str_decode(eventPtr, eventLen);
				if (!element) return;
				const callback = () => wasm_callback(callbackPtr);
				api_events._callbacks.set(callbackPtr, callback);
				element.addEventListener(event, callback);
			},
			event_removeListener: (elementPtr, elementLen, eventPtr, eventLen, callbackPtr) => {
				const element = document.querySelector(str_decode(elementPtr, elementLen));
				const event = str_decode(eventPtr, eventLen);
				const callback = api_events._callbacks.get(callbackPtr);
				if (!element || !callback) return;
				element.removeEventListener(event, callback);
				api_events._callbacks.delete(callbackPtr);
			},
			event_addListenerJs: (elementPtr, elementLen, eventPtr, eventLen, jsFnPtr, jsFnLen) => {
				const element = document.querySelector(str_decode(elementPtr, elementLen));
				const event = str_decode(eventPtr, eventLen);
				const jsFnName = str_decode(jsFnPtr, jsFnLen);
				if (!element) {
					console.error(`Element not found for event '${event}'`);
					return;
				}
				if (typeof window[jsFnName] === "function") {
					const callback = window[jsFnName];
					api_events._callbacks_js.set(jsFnName, callback);
					element.addEventListener(event, callback);
				} else {
					console.error(`JS function '${jsFnName}' not found.`);
				}
			},
			event_removeListenerJs: (elementPtr, elementLen, eventPtr, eventLen, jsFnPtr, jsFnLen) => {
				const element = document.querySelector(str_decode(elementPtr, elementLen));
				const event = str_decode(eventPtr, eventLen);
				const jsFnName = str_decode(jsFnPtr, jsFnLen);

				if (!element) {
					console.error(`Element not found for event '${event}'`);
					return;
				}

				const callback = api_events._callbacks_js.get(jsFnName);
				if (callback) {
					element.removeEventListener(event, callback);
					api_events._callbacks_js.delete(jsFnName);
				} else {
					console.error(`No event listener found for '${jsFnName}' on '${event}'.`);
				}
			},
		},
		api_history_navigation: {
			// History API
			history_back: () => history.back(),
			history_forward: () => history.forward(),
			history_go: (delta) => history.go(delta),
			history_pushState: (statePtr, stateLen, titlePtr, titleLen, urlPtr, urlLen) =>
				history.pushState(str_decode(statePtr, stateLen),
					str_decode(titlePtr, titleLen), str_decode(urlPtr, urlLen)),
			history_replaceState: (statePtr, stateLen, titlePtr, titleLen, urlPtr, urlLen) =>
				history.replaceState(str_decode(statePtr, stateLen),
					str_decode(titlePtr, titleLen), str_decode(urlPtr, urlLen)),
			// Location API
			location_reload: () => location.reload(),
			location_assign: (urlPtr, urlLen) => location.assign(str_decode(urlPtr, urlLen)),
			location_replace: (urlPtr, urlLen) => location.replace(str_decode(urlPtr, urlLen)),
		},
		api_permissions: {
			permissions_query: (namePtr, nameLen) => {
				return navigator.permissions.query({ name: str_decode(namePtr, nameLen) })
					.then(result => {
						switch (result.state) {
							case "granted": return 1;  // JsPermissionState::Granted
							case "denied": return -1;  // JsPermissionState::Denied
							case "prompt": return 0;   // JsPermissionState::Prompt
							default: return -2;        // JsPermissionState::Unknown
						}
					})
					.catch(() => -3); // JsPermissionState::Error
			},
		},

		/* extended APIs*/

		api_canvas: {
			/* custom */
			set_canvas: (ptr, len) => { set_canvas(str_decode(ptr, len)); },

			/* basic drawing */
			fillRect: (x, y, w, h) => ctx.fillRect(x, y, w, h),

			/* color Setting */
			fillStyle: (r, g, b) => ctx.fillStyle = `rgb(${r}, ${g}, ${b})`,
			strokeStyle: (r, g, b) => ctx.strokeStyle = `rgb(${r}, ${g}, ${b})`,

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
			fillText: (ptr, len, x, y) => ctx.fillText(str_decode(ptr, len), x, y),
		},
		api_timing: {
			get_time: () => performance.now(),
		}
	};

	window.api_events = wasmBindings.api_events;
	window.wasm_callback = (callbackPtr) => {
		wasmInstance.exports.wasm_callback(callbackPtr); // Js::wasm_callback
	};

	const finalImports = { ...wasmBindings, ...imports };
	const response = await fetch(wasmPath);
	const wasm = await WebAssembly.instantiateStreaming(response, finalImports);

	wasmInstance = wasm.instance;
	wasm.instance.exports.main();

	return wasmInstance;
}
