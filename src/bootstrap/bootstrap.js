// === General === //
const originalLog = globalThis.console.log;

globalThis.console.log = function (...args) {
  const processedArgs = args.map((arg) => {
    if (arg instanceof Promise) {
      const [status, result] = Deno.core.getPromiseDetails(arg);

      if (status === 0) {
        return "Promise { <pending> }";
      } else if (status === 1) {
        return `Promise { ${typeof result === 'object' ? JSON.stringify(result, null, 2) : result} }`;
      } else {
        return `Promise { <rejected> ${result} }`;
      }
    }

    return arg;
  });

  originalLog.apply(globalThis.console, processedArgs);
};

// === Timers === //
let timerIdCounter = 1;
const cancelledTimers = new Set();

globalThis.setTimeout = (cb, delay, ...args) => {
    const id = timerIdCounter++;
    Deno.core.ops.op_sleep(id, delay ?? 0).then(() => {
        if (cancelledTimers.has(id)) {
            cancelledTimers.delete(id);
            return;
        }
        cb(...args);
    });
    return id;
};

globalThis.clearTimeout = (id) => {
    cancelledTimers.add(id);
    Deno.core.ops.op_cancel_timer(id);
};

globalThis.setInterval = (cb, delay, ...args) => {
    const id = timerIdCounter++;
    const run = () => {
        Deno.core.ops.op_sleep(id, delay ?? 0).then(() => {
            if (cancelledTimers.has(id)) {
                cancelledTimers.delete(id);
                return;
            }
            cb(...args);
            run();
        });
    };
    run();
    return id;
};

globalThis.clearInterval = globalThis.clearTimeout;

// === Fetch === //
globalThis.fetch = async (url, options = {}) => {
    const headers = options.headers ?? {};
    if (!headers["User-Agent"] && !headers["user-agent"]) {
        headers["User-Agent"] = "Mozilla/5.0";
    }
    const raw = await Deno.core.ops.op_fetch(
        url,
        options.method ?? "GET",
        Object.entries(headers),
        options.body ?? null
    );
    const res = JSON.parse(raw);
    return {
        status: res.status,
        ok: res.ok,
        text: () => Promise.resolve(res.body),
        json: () => Promise.resolve(JSON.parse(res.body)),
    };
};
