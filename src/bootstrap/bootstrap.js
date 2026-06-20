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
