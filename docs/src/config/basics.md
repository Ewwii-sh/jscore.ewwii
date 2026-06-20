# Basics

The entry file of jscore is `ewwii.js`. As always, create the entry file (in this case `ewwii.js`), 
and the `ewwii.(s)css` to start your configuration off.

Once the files are created, you can start by first importing the widgets:

```js
import * as Widgets from "ewwii/widgets";
```

This contains all the widgets that you will use to build your configuration.

## Creating A Widget

Let's create a label, shall we? To create a label, call the `Label` function under the `Widgets` namespace like so:

```js 
let myLabel = Widgets.Label();
```

This creates an empty label. To create a label that holds a value, initialize like so:

```js 
let myLabel = Widgets.Label("Hello, World!");
```

Don't worry about widgets taking in parameters too much for now. Only a few really does that, and it mostly is a syntactical sugar.

## Assigning Properties

Now that we have a widget, we should be able to tweak the properties of it. There are two ways in which you can do that.
There is no **right** way to set properties. You can use the method you prefer.

#### 1. Through `properties` function.

```js 
let myLabel = Widgets.Label().properties({
    truncate: true,
    widget_name: "my_label"
});
```

The `properties` function takes in an object (i.e json) which provides a list of properties to set.

#### 2. Through the `set_property` function.

```js 
let myLabel = Widgets.Label();
myLabel.set_property("truncate", true);
myLabel.set_property("widget_name", "my_label");
```

The `set_property` function takes in a key and a value. The key being the name of the property, 
and the value being the value of the property to set.

> Regarding earlier `Widgets.Label("Hello, World!")`.
> 
> ```js 
> let myLabel = Widgets.Label("Hello, World!");
> ```
> 
> Is syntactical sugar for:
>
> ```js 
> let myLabel = Widgets.Label().properties({
>     text: "Hello, World!"
> });
> ```

## Appending Children

Some widgets like `Box` takes in children. You can add a child to these types of 
container widgets using the `add_child` function.

```js 
let myLabel = Widgets.Label("Hello, World!");

let myBox = Widgets.Box();
myBox.add_child(myLabel); // set label as child
```

You would use the same `add_child` property to set the root child of a window.

## Creating a Window

This is the final thing you need to learn to render your first widget. We put together everything we learned till now 
to render a window that shows `"Hello, World!"`.

```js
let myLabel = Widgets.Label("Hello, World!");

let myWindow = Widgets.Window("window_name");
myWindow.add_child(mylabel);
```

Now the final step is to register this window to ewwii so that you can open it with a simple command like `ewwii open window_name`.

```js 
Widgets.register(myWindow);
```

This will register your window to ewwii.

## Full Example

Here is an example code that puts everything together to render a window that shows `"Hello, World!"`:

```js 
import * as Widgets from "ewwii/widgets";

let myLabel = Widgets.Label("Hello, World!");
let myBox = Widgets.Box();

myBox.add_child(myLabel);

let myWindow = Widgets.Window("my_window");
myWindow.add_child(myBox);

Widgets.register(myWindow);
```

Fabulous!
