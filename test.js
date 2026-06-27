import * as Tools from "ewwii/tools";

const handle = Tools.cmd.listen("tail -f file.txt", (line) => {
    // perform operations here
    console.log(line);
});


