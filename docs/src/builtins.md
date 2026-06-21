# Builtins

Jscore is built on top of `deno_core`, which is a very minimal JavaScript evaluation engine which does not support
things like timers, web, and more. To combat this, instead of using `deno_runtime`, which would make the binary size
huge and increase the compile time by a lot, jscore implements most of these by itself instead.

## `fetch(url, {method, headers, body})`

**Supports:**

- `ok`
- `status`
- `text()`
- `json()`

This is the simple fetch implementation of jscore. It is built for simple use cases and not for downloading large files as such.
Simple actions like calling API's and extracting JSON will work seaminglessly.

**Example:**

```js 
// The second param is optional
const response = await fetch("https://api.github.com/repos/ewwii-sh/ewwii");
const json = await response.json();
console.log(json)
```

## Timers

The timers like `setInterval`, `setTimeout`, etc. does not exist in `deno_core`. As these are simple functions,
jscore has them built-in.
