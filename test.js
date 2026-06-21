import * as Tools from "ewwii/tools";

Tools.cmd.run("notify-send Hi");

const output = await Tools.cmd.run_read("echo Hi");
console.log(output); // "Hi"

