import * as Tools from "ewwii/tools";

Tools.cmd.listen("tail -f ~/Desktop/test", (a) => {
    console.log(a)
});

console.log("This should run first");
