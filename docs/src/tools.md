# Tools

Jscore exposes tools to help with you do most of the work from within JavaScript itself. You can imports the tools like so:

```js 
import * as Tools from "ewwii/tools";
```

## cmd

The command namespace. This contains all functions related to commands.

**Methods:**

- `run(cmd)`
- `run_read(cmd)`

**Example:**

```js 
Tools.cmd.run("notify-send Hi");
const output = Tools.cmd.run_read("echo Hi");
```

## fs 

The file system namespace. This contains functions that can modify/interact the file system.

**Methods:**

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
const contents_now = Tools.fs.read(path);

// Replace contents with "Hello, World!"
Tools.fs.write(path, "Hello, World!");

// Add previous content back below "Hello, World!"
Tools.fs.append(path, contents_now);

// Now delete the file
Tools.fs.remove(path);

// Check if it exists
if (Tools.fs.exists(path)) {
    // will not reach
    // as we removed it
}

// Make a directory
const directory = "mydir";
Tools.fs.mkdir(directory);

// read all files (will be empty)
const files = Tools.fs.readdir(directory);

// get info about the dir 
const info = Tools.fs.stat(directory);

// copy/move a dir/file 
Tools.fs.copy(directory, "newdir");
Tools.fs.move("newdir", "newdir_moved");
```
