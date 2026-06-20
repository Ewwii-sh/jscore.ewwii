use ewwii_plugin_api::shared_utils::prop::{PropertyMap, Property};
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub struct WidgetData {
    #[serde(rename = "type")]
    widget_type: String,
    props: Map<String, Value>,
    #[serde(default)]
    children: Vec<WidgetData>
}

pub fn convert_to_widgetnode(json_str: &str) -> Option<WidgetNode> {
    match serde_json::from_str::<WidgetData>(json_str) {
        Ok(widget_tree) => {
            Some(to_node(widget_tree))
        }
        Err(e) => {
            eprintln!("Failed to parse widget JSON framework layout: {}", e);
            None
        }
    }
}

fn to_node(data: WidgetData) -> WidgetNode {
    let mut props = PropertyMap::new();
    for (key, value) in data.props {
        props.insert(key, map_json_value_to_property(value));
    }
    let children = || data.children.into_iter().map(to_node).collect::<Vec<WidgetNode>>();

    match data.widget_type.as_str() {
        "Label" => WidgetNode::Label { props },
        "Box" => WidgetNode::Box { props, children: children() },
        "FlowBox" => WidgetNode::FlowBox { props, children: children() },
        "Button" => WidgetNode::Button { props },
        "Image" => WidgetNode::Image { props },
        "Input" => WidgetNode::Input { props },
        "Progress" => WidgetNode::Progress { props },
        "ComboBoxText" => WidgetNode::ComboBoxText { props },
        "Scale" => WidgetNode::Scale { props },
        "Checkbox" => WidgetNode::Checkbox { props },
        "Expander" => WidgetNode::Expander { props, children: children() },
        "Revealer" => WidgetNode::Revealer { props, children: children() },
        "Scroll" => WidgetNode::Scroll { props, children: children() },
        "OverLay" => WidgetNode::OverLay { props, children: children() },
        "Stack" => WidgetNode::Stack { props, children: children() },
        "Calendar" => WidgetNode::Calendar { props },
        "ColorButton" => WidgetNode::ColorButton { props },
        "ColorChooser" => WidgetNode::ColorChooser { props },
        "CircularProgress" => WidgetNode::CircularProgress { props },
        "Graph" => WidgetNode::Graph { props },
        "Transform" => WidgetNode::Transform { props },
        "EventBox" => WidgetNode::EventBox { props, children: children() },
        "ToolTip" => WidgetNode::ToolTip { props, children: children() },
        "GtkUI" => WidgetNode::GtkUI { props },
        "Script" => WidgetNode::Script { props },
        
        "Window" => {
            let name = match props.get("name").expect("Name is required for a window.") {
                Property::String(s) => s.to_string(),
                _ => {
                    eprintln!("Name must be a string!");
                    String::from("__default_name")
                }
            };
            let first_child = children().into_iter().next().unwrap_or(WidgetNode::Label { props: PropertyMap::default() });
            WidgetNode::DefWindow {
                name,
                props,
                node: Box::new(first_child),
            }
        },
        "Poll" => {
            let var = match props.get("var").expect("Name is required for a poll.") {
                Property::String(s) => s.to_string(),
                _ => {
                    eprintln!("Name must be a string!");
                    String::from("__default_name")
                }
            };
            WidgetNode::Poll { var, props }
        },
        "Listen" => {
            let var = match props.get("var").expect("Name is required for a listen.") {
                Property::String(s) => s.to_string(),
                _ => {
                    eprintln!("Name must be a string!");
                    String::from("__default_name")
                }
            };
            WidgetNode::Listen { var, props }
        },
        
        _ => {
            eprintln!("Warning: Unrecognized widget type '{}', defaulting to Label", data.widget_type);
            WidgetNode::Label { props }
        }
    }
}

fn map_json_value_to_property(value: Value) -> Property {
    match value {
        Value::Null => Property::from(""),
        Value::Bool(b) => Property::from(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Property::from(i)
            } else if let Some(f) = n.as_f64() {
                Property::from(f)
            } else {
                Property::from(n.to_string())
            }
        }
        Value::String(s) => Property::from(s),
        Value::Array(arr) => {
            let props: Vec<Property> = arr.into_iter().map(map_json_value_to_property).collect();
            Property::from(props)
        }
        Value::Object(obj) => {
            let mut map = PropertyMap::default();
            for (k, v) in obj {
                map.insert(k, map_json_value_to_property(v));
            }
            Property::from(map)
        }
    }
}
