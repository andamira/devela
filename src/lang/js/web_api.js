// devela::lang::js::web_api.js
// (in sync with ./web_api.rs)
//
// TOC
// - config
// - helpers
// - bindings
//   - core
//     - console
//     - eval
//     - events
//     - history_navigation
//   - extended
//     - canvas
//     - performance
//     - workers

export async function initWasm(wasmPath, imports = {}) {
	/* Config */

	let wasmInstance;
	let canvas = null;
	let ctx = null;

	/* Helpers */

	// Decode UTF-8 strings from WASM memory
	function str_decode(ptr, len) {
		const memory = new Uint8Array(wasmInstance.exports.memory.buffer, ptr, len);
		return new TextDecoder("utf-8").decode(memory);
	}

	// Sets the active canvas.
	function set_canvas(selector) {
		const newCanvas = document.querySelector(selector);
		if (!newCanvas) { console.error(`Canvas with ID "${canvasId}" not found`); return; }
		const newCtx = newCanvas.getContext("2d");
		if (!newCtx) { console.error(`Failed to get 2D context for canvas "${canvasId}"`); return; }
		canvas = newCanvas;
		ctx = newCtx;
	}

	/* Bindings */

	const wasmBindings = {

		/* Core APIs */

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
		api_eval: {
			eval: (jsCodePtr, jsCodeLen) => {
				const jsCode = str_decode(jsCodePtr, jsCodeLen);
				try { eval(jsCode); }
				catch (err) { console.error("Error evaluating JavaScript:", err); }
			},
			eval_timeout: (jsCodePtr, jsCodeLen, delayMs) => {
				const jsCode = str_decode(jsCodePtr, jsCodeLen);
				return setTimeout(() => eval(jsCode), delayMs);
			},
			eval_interval: (jsCodePtr, jsCodeLen, intervalMs) => {
				const jsCode = str_decode(jsCodePtr, jsCodeLen);
				return setInterval(() => eval(jsCode), intervalMs);
			},
			eval_timeout_clear: (id) => { clearTimeout(id); },
			eval_interval_clear: (id) => { clearInterval(id); }
		},
		api_events: {
			// Events API
			_callbacks: new Map(),
			event_addListener: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
				const element = document.querySelector(str_decode(ePtr, eLen));
				const event = str_decode(eventPtr, eventLen);
				if (!element) return;
				const callback = () => wasm_callback(callbackPtr);
				api_events._callbacks.set(callbackPtr, callback);
				element.addEventListener(event, callback);
			},
			event_removeListener: (ePtr, eLen, eventPtr, eventLen, callbackPtr) => {
				const element = document.querySelector(str_decode(ePtr, eLen));
				const event = str_decode(eventPtr, eventLen);
				const callback = api_events._callbacks.get(callbackPtr);
				if (!element || !callback) return;
				element.removeEventListener(event, callback);
				api_events._callbacks.delete(callbackPtr);
			},
			event_addListenerJs: (ePtr, eLen, eventPtr, eventLen, jsFnPtr, jsFnLen) => {
				const element = document.querySelector(str_decode(ePtr, eLen));
				const event = str_decode(eventPtr, eventLen);
				const jsFnName = str_decode(jsFnPtr, jsFnLen);
				if (!element) { console.error(`Element not found for event '${event}'`); return; }
				if (typeof window[jsFnName] === "function") {
					const callback = window[jsFnName];
					api_events._callbacks_js.set(jsFnName, callback);
					element.addEventListener(event, callback); }
				else { console.error(`JS function '${jsFnName}' not found.`); }
			},
			event_removeListenerJs: (ePtr, eLen, eventPtr, eventLen, jsFnPtr, jsFnLen) => {
				const element = document.querySelector(str_decode(ePtr, eLen));
				const event = str_decode(eventPtr, eventLen);
				const jsFnName = str_decode(jsFnPtr, jsFnLen);
				if (!element) { console.error(`Element not found for event '${event}'`); return; }
				const callback = api_events._callbacks_js.get(jsFnName);
				if (callback) {
					element.removeEventListener(event, callback);
					api_events._callbacks_js.delete(jsFnName); }
				else { console.error(`No event listener found for '${jsFnName}' on '${event}'.`); }
			},
		}, // api_events
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
						switch (result.state) { // JsPermissionState::
							case "granted": return 1; // Granted
							case "prompt": return 0;  // Prompt
							case "denied": return -1; // Denied
							default: return -2;       // Unknown
						}}).catch(() => -3); // Error
			},
		},

		/* Extended APIs*/

		api_canvas: {
			/* misc. */
			set_canvas: (ptr, len) => { set_canvas(str_decode(ptr, len)); },
			/* color settings */
			fillStyle: (r, g, b) => ctx.fillStyle = `rgb(${r}, ${g}, ${b})`,
			strokeStyle: (r, g, b) => ctx.strokeStyle = `rgb(${r}, ${g}, ${b})`,
			/* drawing rectangles */
			fillRect: (x, y, w, h) => ctx.fillRect(x, y, w, h),
			strokeRect: (x, y, w, h) => ctx.strokeRect(x, y, w, h),
			clearRect: (x, y, w, h) => ctx.clearRect(x, y, w, h),
			/* drawing paths */
			/* drawing shapes */
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
			/* drawing text */
			fillText: (ptr, len, x, y) => ctx.fillText(str_decode(ptr, len), x, y),
			strokeText: (ptr, len, x, y) => ctx.strokeText(str_decode(ptr, len), x, y),
			measureText: (textPtr, textLen, outMetricsPtr) => {
				const text = str_decode(textPtr, textLen);
				const metrics = ctx.measureText(text);
				const outMetrics
					= new Float32Array(wasmInstance.exports.memory.buffer, outMetricsPtr, 3);
				outMetrics[0] = metrics.width;
				outMetrics[1] = metrics.actualBoundingBoxAscent;
				outMetrics[2] = metrics.actualBoundingBoxDescent;
			},
			measureTextFull: (textPtr, textLen, outMetricsPtr) => {
				const text = str_decode(textPtr, textLen);
				const metrics = ctx.measureText(text);
				const outMetrics
					= new Float32Array(wasmInstance.exports.memory.buffer, outMetricsPtr, 12);
				outMetrics[0] = metrics.width;
				outMetrics[1] = metrics.actualBoundingBoxLeft;
				outMetrics[2] = metrics.actualBoundingBoxRight;
				outMetrics[3] = metrics.actualBoundingBoxAscent;
				outMetrics[4] = metrics.actualBoundingBoxDescent;
				outMetrics[5] = metrics.fontBoundingBoxAscent ?? 0.0;
				outMetrics[6] = metrics.fontBoundingBoxDescent ?? 0.0;
				outMetrics[7] = metrics.emHeightAscent ?? 0.0;
				outMetrics[8] = metrics.emHeightDescent ?? 0.0;
				outMetrics[9] = metrics.hangingBaseline ?? 0.0;
				outMetrics[10] = metrics.alphabeticBaseline ?? 0.0;
				outMetrics[11] = metrics.ideographicBaseline ?? 0.0;
			},
		}, // api_canvas
		api_performance: {
			now: () => performance.now(),
			timeOrigin: () => performance.timeOrigin,
			eventCounts: (eventPtr, eventLen) => {
				const eventName = str_decode(eventPtr, eventLen);
				return performance.eventCounts?.get([eventName]) ?? 0;
			}
		},
		api_workers: {
			_workers: new Map(),      // Map of worker_id -> Worker instance
			_nextWorkerId: 1,         // Auto-incrementing worker ID
			_jobQueue: new Map(),     // Job ID -> Promise resolution mapping
			_messageQueue: new Map(), // Worker ID -> Message Handlers

			// Spawns a new worker and returns its unique ID.
			worker_spawn: (script_ptr, script_len) => {
				let script = str_decode(script_ptr, script_len);
				let worker;
				if (script.startsWith("function") || script.includes("self.onmessage")) {
					const blob = new Blob([script], { type: "application/javascript" });
					worker = new Worker(URL.createObjectURL(blob));
				} else { // then it's a file to load
					worker = new Worker(script);
				}
				const wid = wasmBindings.api_workers._nextWorkerId++;
				worker.onmessage = (event) => wasmBindings.api_workers._handleMessage(wid, event);
				worker.onerror = (error) => console.error(`Worker ${wid} error:`, error);
				wasmBindings.api_workers._workers.set(wid, worker);
				return wid;
			},
			// Stops a specific worker by ID.
			worker_stop: (worker_id) => {
				const worker = wasmBindings.api_workers._workers.get(worker_id);
				if (worker) {
					worker.terminate();
					wasmBindings.api_workers._workers.delete(worker_id);
				}
			},
			// Stops all active workers.
			worker_stop_all: () => {
				wasmBindings.api_workers._workers.forEach(worker => worker.terminate());
				wasmBindings.api_workers._workers.clear();
			},
			// Returns the number of active workers.
			worker_list_len: () => { return wasmBindings.api_workers._workers.size; },
			// Write worker IDs into the Rust buffer and returns the number of IDs written
			worker_list: (buf_ptr, buf_len) => {
				const workers = Array.from(wasmBindings.api_workers._workers.keys());
				const count = Math.min(workers.length, buf_len);
				const buffer = new Uint32Array(wasmInstance.exports.memory.buffer, buf_ptr, buf_len);
				for (let i = 0; i < count; i++) {
					buffer[i] = workers[i];
				}
				return count;
			},
			// Sends a message to a worker.
			worker_send_message: (worker_id, msg_ptr, msg_len) => {
				const message = str_decode(msg_ptr, msg_len);
				const worker = wasmBindings.api_workers._workers.get(worker_id);
				if (!worker) { console.error(`Worker ${worker_id} not found.`); return; }
				worker.postMessage({ type: "message", message });
			},
			// Handles messages received from workers.
			_handleMessage: (worker_id, event) => {
				if (event.data.type === "message") {
					const handler = wasmBindings.api_workers._messageQueue.get(worker_id);
					if (handler) {
						handler(event.data.message);
					}
				} else if (event.data.type === "eval_result") {
					const { jobId, result } = event.data;
					const resolve = wasmBindings.api_workers._jobQueue.get(jobId);
					if (resolve) {
						resolve(result);
						wasmBindings.api_workers._jobQueue.delete(jobId);
					}
				}
			},
			// Runs JavaScript inside a specific Web Worker or the main thread.
			worker_eval: (worker_id, jobId, jsCodePtr, jsCodeLen) => {
				const jsCode = str_decode(jsCodePtr, jsCodeLen);
				if (worker_id === 0) {
					// Run JS in the main thread
					try {
						const result = eval(jsCode);
						return result;
					} catch (error) {
						console.error("Eval error:", error);
						return `Error: ${error.message}`;
					}
				} else {
					// Run JS in a worker
					return new Promise((resolve) => {
						const worker = wasmBindings.api_workers._workers.get(worker_id);
						if (!worker) {
							console.error(`Worker ${worker_id} not found.`);
							return;
						}
						wasmBindings.api_workers._jobQueue.set(jobId, resolve);
						worker.postMessage({ type: "eval", jobId, jsCode });
					});
				}
			}
		},
	};

	/* Global Namespace Setup */

	// Make Web API modules globally accessible from Rust
	window.api_events = wasmBindings.api_events;
	window.api_workers = wasmBindings.api_workers;

	// Allows Rust to call JavaScript functions via function pointers (Js::wasm_callback)
	window.wasm_callback = (callbackPtr) => { wasmInstance.exports.wasm_callback(callbackPtr); };

	/* WASM Instantiation */

	// Combine default bindings with additional imports from Rust
	const finalImports = { ...wasmBindings, ...imports };

	try { // Fetch and instantiate the WebAssembly binary
		const response = await fetch(wasmPath);
		if (!response.ok) throw new Error(`Failed to load WASM: ${response.statusText}`);
		const wasm = await WebAssembly.instantiateStreaming(response, finalImports);
		wasmInstance = wasm.instance;
		wasm.instance.exports.main();
		return wasmInstance;
	} catch (error) { console.error("WASM loading failed:", error); return null; }

	wasmInstance = wasm.instance; // Store the WebAssembly instance globally
	wasm.instance.exports.main(); // Run the extern Rust `main()` function
	return wasmInstance;
}
