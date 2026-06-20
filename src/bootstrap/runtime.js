class WidgetAPI {
    find(widgetId) {
        return new LiveWidgetHandle(widgetId);
    }
}

class LiveWidgetHandle {
    constructor(windowName, widgetId) {
        this.widgetId = widgetId;
    }

    set_property(key, value) {
        Deno.core.ops.op_update_widget_property(this.widgetId, key, value);
        return this;
    }

    add_class(className) {
        Deno.core.ops.op_widget_add_class(this.widgetId, className);
        return this;
    }

    add_classes(...classNames) {
        for (const name of classNames) {
            this.add_class(name);
        }
        return this;
    }

    remove_class(className) {
        Deno.core.ops.op_widget_remove_class(this.widgetId, className);
        return this;
    }

    remove_classes(...classNames) {
        for (const name of classNames) {
            this.remove_class(name);
        }
        return this;
    }
}

globalThis.WidgetAPI = WidgetAPI;


