// Core of "ewwii/widgets" loader
class Widget {
    constructor(type, defaultProp = null, defaultPropName = null, max_child = null) {
        this.type = type;
        this.props = {};
        this.children = [];
        this.max_child = max_child;

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

    toJSON() {
        return {
            type: this.type,
            props: this.props,
            children: this.children.length > 0 ? this.children : undefined
        };
    }
}

// User exposed stuff
export const Label = (text) => new Widget("Label", text, "text", 0);
export const Box = () => new Widget("Box");
export const Window = (name) => {
    if (!name) {
        throw new TypeError("Window requires a name.");
    }
    return new Widget("Window", name, "name", 1);
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

    Deno.core.ops.op_register_window_json(JSON.stringify(widgetInstance));
}
