// Test setTimeout
setTimeout(() => console.log("setTimeout: fired after 500ms"), 500);

// Test clearTimeout
const id = setTimeout(() => console.log("clearTimeout: this should NOT print"), 50);
clearTimeout(id);

// Test setInterval
let count = 0;
const intervalId = setInterval(() => {
    count++;
    console.log(`setInterval: tick ${count}`);
    if (count >= 3) {
        clearInterval(intervalId);
        console.log("setInterval: cleared after 3 ticks");
    }
}, 300);

console.log("sync: this should print first");
