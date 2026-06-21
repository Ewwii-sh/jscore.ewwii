// === General === //
const originalMethods = {
    log: globalThis.console.log,
    warn: globalThis.console.warn,
    error: globalThis.console.error
};

function cleanInspect(value) {
    if (value instanceof Error) {
        return value.stack || value.message;
    }

    if (typeof value !== 'object' || value === null) {
        return value;
    }

    return JSON.stringify(value, (key, val) => {
        if (typeof val === 'function') {
            return `[Function: ${val.name || '(anonymous)'}]`;
        }
        return val;
    }, 2);
}

for (const method of ['log', 'warn', 'error']) {
    globalThis.console[method] = function (...args) {
        const processedArgs = args.map((arg) => {
            if (arg instanceof Promise) {
                const [status, result] = Deno.core.getPromiseDetails(arg);

                if (status === 0) {
                    return "Promise { <pending> }";
                } else if (status === 1) {
                    return `Promise { ${cleanInspect(result)} }`;
                } else {
                    return `Promise { <rejected> ${result} }`;
                }
            }

            if (typeof arg === 'object' && arg !== null) {
                return cleanInspect(arg);
            }

            return arg;
        });

        originalMethods[method].apply(globalThis.console, processedArgs);
    };
}

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

// === Encoder/Decoder === //
class TextEncoder {
    encode(string) {
        const bytes = new Uint8Array(string.length);

        for (let i = 0; i < string.length; i++) {
            const code = string.charCodeAt(i);
            bytes[i] = code;
        }

        return bytes;
    }

    encodeInto(string, buffer) {
        let read = 0;
        let written = 0;

        while (read < string.length && written < buffer.length) {
            const code = string.charCodeAt(read);
            buffer[written] = code;

            read++;
            written++;
        }

        return {
            read: read,
            written: written
        };
    }
}

class TextDecoder {
    decode(buffer) {
        let string = '';

        for (let i = 0; i < buffer.length; i++) {
            const byte = buffer[i];
            const char = String.fromCharCode(byte);

            string += char;
        }

        return string;
    }
}

globalThis.TextEncoder = TextEncoder;
globalThis.TextDecoder = TextDecoder;
