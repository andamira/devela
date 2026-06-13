// devela/src/sys/os/browser/web/js/workers.js
// (in sync with ../api/workers.rs)

import { strDecode, strEncode } from "./shared.js";

function isInlineWorkerScript(script) {
  return script.startsWith("function")
    || script.includes("self.onmessage")
    || script.includes("addEventListener(\"message\"")
    || script.includes("addEventListener('message'");
}

export function makeWorkersApi(env) {
  const workers = new Map();     // worker_id -> Worker
  const evalResults = new Map(); // job_id -> null | string
  let nextWorkerId = 1;
  let nextJobId = 1;

  // Handles messages received from workers.
  function handleMessage(workerId, event) {
    const data = event.data;
    if (!data || typeof data !== "object") {
      console.log(`Worker ${workerId} message:`, data);
      return;
    }
    if (data.type === "eval_result") {
      evalResults.set(data.jobId, String(data.result));
    } else if (data.type === "message_response") {
      console.log(`Worker ${workerId} response: ${data.message}`); // IMPROVE
    } else {
      console.log(`Worker ${workerId} message:`, data);
    }
  }

  return {
    // Spawns a new worker and returns its unique ID.
    worker_spawn: (scriptPtr, scriptLen) => {
      const script = strDecode(env, scriptPtr, scriptLen);
      let worker;
      if (isInlineWorkerScript(script)) {
        const blob = new Blob([script], { type: "application/javascript" });
        const url = URL.createObjectURL(blob);
        try {
          worker = new Worker(url);
        } catch (error) {
          console.error("Failed to spawn inline worker:", error);
          URL.revokeObjectURL(url);
          return 0;
        }
        URL.revokeObjectURL(url);
      } else {
        try {
          worker = new Worker(script);
        } catch (error) {
          console.error(`Failed to spawn worker from '${script}':`, error);
          return 0;
        }
      }
      const workerId = nextWorkerId++;
      worker.onmessage = (event) => handleMessage(workerId, event);
      worker.onerror = (error) => {
        console.error(`Worker ${workerId} error:`, error);
      };
      workers.set(workerId, worker);
      return workerId;
    },

    // Returns `true` if the given worker is active.
    worker_is_active: (workerId) => { return workers.has(workerId); },

    // Stops a specific worker by ID.
    worker_stop: (workerId) => {
      const worker = workers.get(workerId);
      if (!worker) return;
      worker.terminate();
      workers.delete(workerId);
    },

    // Stops all active workers.
    worker_stop_all: () => {
      workers.forEach(worker => worker.terminate());
      workers.clear();
    },

    // Returns the number of active workers.
    worker_list_len: () => { return workers.size; },

    // Write worker IDs into the Rust buffer and returns the number of IDs written
    worker_list: (bufPtr, bufLen) => {
      const ids = Array.from(workers.keys());
      const count = Math.min(ids.length, bufLen);
      const out = new Uint32Array(env.wasm.exports.memory.buffer, bufPtr, count);
      for (let i = 0; i < count; i++) {
        out[i] = ids[i];
      }
      return count;
    },

    // Sends a message to a worker.
    worker_send_message: (workerId, msgPtr, msgLen) => {
      const worker = workers.get(workerId);
      if (!worker) {
        console.error(`Worker ${workerId} not found.`);
        return;
      }
      const message = strDecode(env, msgPtr, msgLen);
      worker.postMessage({ type: "message", message });
    },

    // Runs JavaScript inside a worker, and returns the JobId or 0 to indicate failure.
    worker_eval: (workerId, jsCodePtr, jsCodeLen) => {
      const worker = workers.get(workerId);
      if (!worker) {
        console.error(`Worker ${workerId} not found.`);
        return 0;
      }
      const jsCode = strDecode(env, jsCodePtr, jsCodeLen);
      const jobId = nextJobId++;
      evalResults.set(jobId, null);
      worker.postMessage({ type: "eval", jobId, jsCode });
      return jobId;
    },

    // Polls for the evaluation result and writes it into a buffer.
    worker_poll: (jobId, bufPtr, bufLen) => {
      if (!evalResults.has(jobId)) return -1;

      const result = evalResults.get(jobId);
      if (result === null) return 0;

      evalResults.delete(jobId);

      const out = new Uint8Array(env.wasm.exports.memory.buffer, bufPtr, bufLen);
      const encoded = strEncode(env, result);
      const bytesWritten = Math.min(encoded.length, bufLen);

      out.set(encoded.subarray(0, bytesWritten));

      return bytesWritten;
    },
  };
}
