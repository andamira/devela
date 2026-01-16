// devela::examples::sys::web_worker::public_html::worker.js
//
//! Worker script that evaluates JavaScript and handles messages
//

console.info(">>> Worker 1 started from file is executing!");

try {
    console.log(">>> Worker 1 file loaded and running!");
} catch (error) {
    console.error(">>> Worker 1 failed to start:", error);
}

self.onmessage = (event) => {
    console.log(">>> Worker 1 received message:", event.data);

    const { type, jobId, jsCode, message } = event.data;

    if (type === "eval") {
        console.log(`>>> Worker 1 executing job ${jobId}:`, jsCode);

        try {
            const result = eval(jsCode);
            console.log(`>>> Worker 1 sending result for job ${jobId}:`, result);
            self.postMessage({ type: "eval_result", jobId, result });
        } catch (error) {
            console.error(">>> Worker 1 eval error:", error);
            self.postMessage({ type: "eval_result", jobId, result: `Error: ${error.message}` });
        }
    } else if (type === "message") {
        console.log(">>> Worker 1 received message:", message);
        self.postMessage({ type: "message_response", message: `Worker echoes: ${message}` });
    }
};
