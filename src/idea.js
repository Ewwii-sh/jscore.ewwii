import * as Widgets from "ewwii/widgets";

// Definition
let myBox = Widgets.Box().properties({
    orientation: "v",
    widget_name: "cool-box"
});

let myLabel = Widgets.Label("Hello");
myLabel.set_property("truncate", "true");

myBox.children.push(myLabel);

let myWindow = Widgets.Window("bar").properties({
    exclusive: true
});
myWindow.children.push(myBox);

// Registration
Widgets.register(myWindow);

// == Driving Widgets ==
export function after_render(api) {
    // drive the widgets post render 
    const widget = api.find("cool-box");

    // example use of API:
    widget.set_property("orientation", "h");

    widget.add_class("red");
    widget.add_classes("green", "blue", "orange");

    widget.remove_class("green");
    widget.remove_classes("red", "blue", "orange");
}
