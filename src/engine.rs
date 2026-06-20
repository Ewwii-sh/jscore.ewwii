use ewwii_plugin_api::{EwwiiAPI, IpcRequest, WidgetControlType};
use deno_core::{op2, OpState, v8, JsRuntime, RuntimeOptions};
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use crate::resolver::CustomResolver;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

#[op2(fast)]
fn op_register_window_json(state: &mut OpState, #[string] json: &str) {
    if let Some(widget_state_arc) = state.try_borrow_mut::<Arc<Mutex<WidgetNode>>>() {
        let mut tree = widget_state_arc.lock().unwrap();

        match &mut *tree {
            WidgetNode::Tree(tree) => {
                if let Some(wnode) = crate::convert::convert_to_widgetnode(json) {
                    tree.push(wnode);
                }
            }
            _ => {
                eprintln!("Weirdly, WidgetNode is not a tree.");
                return;
            }
        }
    } 
}

#[op2(fast)]
fn op_update_widget_property(state: &mut OpState, #[string] widget: &str, #[string] name: &str, #[string] value: &str) {
    if let Some(Some(host)) = state.try_borrow_mut::<Option<Arc<dyn EwwiiAPI>>>() {
        host.ipc_request(IpcRequest::WidgetControl(WidgetControlType::PropertyUpdate {
            widget: widget.to_string(),
            prop: name.to_string(),
            value: value.to_string()
        }));
    }
}

#[op2(fast)]
fn op_widget_add_class(state: &mut OpState, #[string] widget: &str, #[string] class: &str) {
    if let Some(Some(host)) = state.try_borrow_mut::<Option<Arc<dyn EwwiiAPI>>>() {
        host.ipc_request(IpcRequest::WidgetControl(WidgetControlType::AddClass {
            widget: widget.to_string(),
            class: class.to_string(),
        }));
    }
}

#[op2(fast)]
fn op_widget_remove_class(state: &mut OpState, #[string] widget: &str, #[string] class: &str) {
    if let Some(Some(host)) = state.try_borrow_mut::<Option<Arc<dyn EwwiiAPI>>>() {
        host.ipc_request(IpcRequest::WidgetControl(WidgetControlType::RemoveClass {
            widget: widget.to_string(),
            class: class.to_string()
        }));       
    }
}

deno_core::extension!(
    jscore_extension,
    ops = [op_register_window_json, op_update_widget_property, op_widget_add_class, op_widget_remove_class]
);

pub static BOOTSTRAP_JS: &str = include_str!("bootstrap/bootstrap.js");
pub static RUNTIME_JS: &str = include_str!("bootstrap/runtime.js");

pub struct Engine {
    widget_state: Arc<Mutex<WidgetNode>>,
    host: Option<Arc<dyn EwwiiAPI>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            widget_state: Arc::new(Mutex::new(WidgetNode::Tree(vec![]))),
            host: None,
        }
    }

    pub fn set_host(&mut self, host: Arc<dyn EwwiiAPI>) {
        self.host = Some(host);
    }

    pub fn start_engine(&self, user_js_code: &str, user_js_path: &str) {
        let mut runtime_opts = RuntimeOptions::default();
        runtime_opts.extensions = vec![jscore_extension::init()];
        runtime_opts.module_loader = Some(Rc::new(CustomResolver::new()));
        runtime_opts.create_params = Some(
            v8::Isolate::create_params()
                .heap_limits(0, 128 * 1024 * 1024)
        );

        let mut runtime = JsRuntime::new(runtime_opts);
        runtime.execute_script("__bootstrap.js", BOOTSTRAP_JS).unwrap();
        runtime.execute_script("__runtime.js", RUNTIME_JS).unwrap();

        let op_state = runtime.op_state();
        op_state.borrow_mut().put(self.widget_state.clone());
        op_state.borrow_mut().put(self.host.clone());

        futures::executor::block_on(async {
            let module_specifier = deno_core::ModuleSpecifier::parse(&format!("file:///{}", user_js_path)).unwrap();

            let module_id = match runtime.load_side_es_module_from_code(&module_specifier, user_js_code.to_string()).await {
                Ok(id) => id,
                Err(err) => {
                    eprintln!("JavaScript Compilation Error: Check your syntax or imports!\nDetails: {}", err);
                    return;
                }
            };

            let evaluation_result = runtime.mod_evaluate(module_id);
            if let Err(err) = runtime.run_event_loop(Default::default()).await {
                eprintln!("Runtime Event Loop Error: {}", err);
                return;
            }

            if let Err(err) = evaluation_result.await {
                eprintln!("Uncaught JavaScript Exception: {}", err);
                return;
            }

            trigger_after_render_lifecycle(&mut runtime, module_id, "main").await;
            let _ = runtime.run_event_loop(Default::default()).await;
        });
    }

    pub fn get_widgetnode(&self) -> Arc<Mutex<WidgetNode>> {
        self.widget_state.clone()
    }
}

async fn trigger_after_render_lifecycle(runtime: &mut JsRuntime, module_id: deno_core::ModuleId, window_name: &str) {
    let module_namespace = runtime.get_module_namespace(module_id).expect("Failed to get module namespace");
    let global_ctx = runtime.main_context();
    let isolate = runtime.v8_isolate();

    v8::scope_with_context!(scope, isolate, global_ctx);
    let module_obj = v8::Local::new(scope, module_namespace);

    let key = v8::String::new(scope, "after_render").unwrap();
    if let Some(exported_value) = module_obj.get(scope, key.into()) {
        if exported_value.is_function() {
            let function: v8::Local<v8::Function> = exported_value.try_into().unwrap();

            let script_source = v8::String::new(scope, &format!("new WidgetAPI('{}')", window_name)).unwrap();
            let script = v8::Script::compile(scope, script_source, None).unwrap();
            let api_instance = script.run(scope).unwrap();

            let receiver = v8::undefined(scope).into();
            let args = [api_instance];

            match function.call(scope, receiver, &args) {
                Some(_) => {},
                None => eprintln!("Runtime Error occurred during after_render script execution."),
            }
        } else {
            println!("Lifecycle Warning: 'after_render' export is missing or not a function. Skipping execution.");
        }
    }
}
