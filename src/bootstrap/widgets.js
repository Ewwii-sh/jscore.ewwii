// Core of "ewwii/widgets" loader
class Widget {
    constructor(type, defaultProp = null, defaultPropName = null, max_child = null, min_child = 0) {
        this.type = type;
        this.props = {};
        this.children = [];
        this.max_child = max_child;
        this.min_child = min_child;

        if (defaultProp && defaultPropName) {
            this.props[defaultPropName] = defaultProp;
        }
    }

    properties(obj) {
        Object.assign(this.props, obj);
        return this;
    }

    set_property(key, value) {
        this.props[key] = value;
        return this;
    }

    add_child(child) {
        if (this.max_child !== null && this.children.length >= this.max_child) {
            throw new Error(`Widget '${this.type}' can hold a maximum of ${this.max_child} child nodes.`);
        }
        this.children.push(child);
        return this;
    }

    validate() {
        if (this.children.length < this.min_child) {
            throw new Error(`Widget '${this.type}' requires a minimum of ${this.min_child} child nodes, but only has ${this._children.length}.`);
        }
        
        for (const child of this.children) {
            if (child instanceof Widget) {
                child.validate();
            }
        }
    }

    toJSON() {
        return {
            type: this.type,
            props: this.props,
            children: this.children.length > 0 ? this.children : undefined
        };
    }
}

// User exposed stuff

// containers
export const Box = () => new Widget("Box");
export const FlowBox = () => new Widget("FlowBox");
export const Expander = () => new Widget("Expander", null, null, 1, 1);
export const Revealer = () => new Widget("Revealer", null, null, 1, 0);
export const Scroll = () => new Widget("Scroll", null, null, 1, 1);
export const OverLay = () => new Widget("OverLay", null, null, null, 1);
export const Stack = () => new Widget("Stack");
export const EventBox = () => new Widget("EventBox", null, null, 1, 1);
export const ToolTip = () => new Widget("ToolTip", null, null, 2, 2);

// leaf widgets
export const Label = (text) => new Widget("Label", text, "text", 0, 0);
export const Button = (text) => new Widget("Button", text, "text", 0, 0);
export const Image = (path) => new Widget("Image", path, "path", 0, 0);
export const Input = () => new Widget("Input", null, null, 0, 0);
export const Progress = () => new Widget("Progress", null, null, 0, 0);
export const ComboBoxText = () => new Widget("ComboBoxText", null, null, 0, 0);
export const Scale = () => new Widget("Scale", null, null, 0, 0);
export const Checkbox = () => new Widget("Checkbox", null, null, 0, 0);
export const Calendar = () => new Widget("Calendar", null, null, 0, 0);
export const ColorButton = () => new Widget("ColorButton", null, null, 0, 0);
export const ColorChooser = () => new Widget("ColorChooser", null, null, 0, 0);
export const CircularProgress = () => new Widget("CircularProgress", null, null, 0, 0);
export const Graph = () => new Widget("Graph", null, null, 0, 0);
export const Transform = () => new Widget("Transform", null, null, 0, 0);

// special widgets
export const GtkUI = () => new Widget("GtkUI", null, null, 0, 0);

// top level
export const Window = (name) => {
    if (!name) {
        throw new TypeError("Window requires a name.");
    }
    return new Widget("Window", name, "name", 1, 1);
}

export function register(widgetInstance_raw) {
    if (!widgetInstance_raw) {
        throw new TypeError("register() requires a valid widget instance argument.");
    }

    const widgetInstance = widgetInstance_raw.toJSON();

    if (widgetInstance.type != "Window") {
        throw new TypeError("Widget Instance must be of type 'Window'.");
    }

    if (widgetInstance.children == undefined) {
        throw new TypeError("Window must have a child assigned to it.")
    }

    widgetInstance_raw.validate();
    Deno.core.ops.op_register_window_json(JSON.stringify(widgetInstance));
}
