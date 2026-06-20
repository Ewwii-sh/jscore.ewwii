import * as Widgets from "ewwii/widgets";

console.log("JS is running!");

let label = Widgets.Label().properties({
    text: "Hello, World!",
    truncate: true,
});
let win = Widgets.Window("a");
win.add_child(label);

Widgets.register(win);
