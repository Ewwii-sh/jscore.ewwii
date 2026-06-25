# Tools

Jscore exposes tools to help with you do most of the work from within JavaScript itself. You can imports the tools like so:

```js 
import * as Tools from "ewwii/tools";
```

## stream

The stream namespace. This contains functions that can stream data to a closure. **Useful alongisde after_render**.
The functions return an object that has the `stop` method, which when called stops the stream.

**Methods:**

All the methods are **sync**.

- `time`

**Example:**

```js 
export function after_render(api) {
    const myLabel = api.find("cool-text");

    let handle = Tools.stream.time((t) => {
        console.log(`System ticked at: ${t.iso}`);
        myLabel.set_property("text", t.string);
    });

    // handle.stop() -> stops the stream
}
```

## cmd

The command namespace. This contains all functions related to commands.

**Methods:**

All methods are **async**.

- `run(cmd)`
- `run_read(cmd)`

**Example:**

```js 
await Tools.cmd.run("notify-send Hi");

const output = await Tools.cmd.run_read("echo Hi");
console.log(output); // "Hi"
```

## fs 

The file system namespace. This contains functions that can modify/interact the file system.

**Methods:**

All methods are **async**.

- `read(path)`
- `write(path, content)`
- `append(path, contnet)`
- `remove(path)`
- `exists(path)`
- `mkdir(path)`
- `readdir(path)`
- `stat(path)`
- `copy(src, dest)`
- `move(src, dest)`

**Example:**

```js 
const path = "./example.js";

// Read to a variable
const contents_now = await Tools.fs.read(path);

// Replace contents with "Hello, World!"
await Tools.fs.write(path, "Hello, World!");

// Add previous content back below "Hello, World!"
await Tools.fs.append(path, contents_now);

// Now delete the file
await Tools.fs.remove(path);

// Check if it exists
if (Tools.fs.exists(path)) {
    // will not reach
    // as we removed it
}

// Make a directory
const directory = "mydir";
await Tools.fs.mkdir(directory);

// read all files (will be empty)
const files = await Tools.fs.readdir(directory);

// get info about the dir 
const info = await Tools.fs.stat(directory);
console.log(info);

// copy/move a dir/file 
await Tools.fs.copy(directory, "newdir");
await Tools.fs.move("newdir", "newdir_moved");
```

