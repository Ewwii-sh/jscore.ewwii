// structuredClone
const obj = { a: 1, b: { c: 2 } };
const cloned = Deno.core.ops.op_structured_clone(obj);
console.log(cloned);

// queueMicrotask
Deno.core.ops.op_queue_microtask(() => console.log("microtask ran"));
console.log("this should print first");
