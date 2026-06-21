# Driving Widgets

This is how you update a widget after rendering in jscore. This is jscore's alternative to `Poll` and `Listen`. 
Jscore takes advantage of the superpower of the `ewwii wc` command and makes it the default way to update widgets
through an API that abstracts all the complexity away.

## Exposing a Widget 

First, let's learn how a widget can qualify to be updated after rendering. It just simply has to have a `widget_name` 
property set.

```js 
let myBox = Widgets.Box().properties({
    widget_name: "cool-box"
});
```

That's it! `ewwii wc` can now find the widget, which this whole feature is based on.

## Setup

To drive a widget render render, you simply need to define a function named `after_render` and export it.

```js 
export function after_render(api) {}
```

This function should take exactly one parameter, that is the API which you will use.

## API

The `api` parameter that is passed to the `after_render` function only has one method. That is `find`. 
You can use the `find` method to find a widget with a specific `widget_name` and update it.

```js 
export function after_render(api) {
    const widget = api.find("cool-box");
}
```

Now the `widget` variable holds all the API's that can directly update the widget with the name `cool-box`.
The `widget` variable holds these methods: 

- `set_property`
- `add_class`
- `remove_class`
- `add_classes`
- `remove_classes`

Signature of these methods:

```js 
set_property(string, string);

add_class(string);
remove_class(string);

// array of strings
add_classes([string]); 
remove_classes([string]);
```

## Full Example

Here is an example that uses all the API's we've discussed.

```js
export function after_render(api) {
    // suppose we have a widget 
    // with name 'cool-box'
    const widget = api.find("cool-box");

    widget.set_property("orientation", "h");

    widget.add_class("red");
    widget.add_classes("green", "blue", "orange");

    widget.remove_class("green");
    widget.remove_classes("red", "blue", "orange");
}
```

Do note that all of these runs all at once. Ideally, you'd use `async` combined with other functions to update widgets programmically.
Jscore also has a `stream` namespace under [Tools](../tools.md) that provides predefined functions to make updating widgets easier.
