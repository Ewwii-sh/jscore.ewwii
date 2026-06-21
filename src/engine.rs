use crate::ext::{jscore_cmd, jscore_fetch, jscore_fs, jscore_timers};
use crate::resolver::CustomResolver;
use deno_core::{JsRuntime, OpState, RuntimeOptions, op2, v8};
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use ewwii_plugin_api::{EwwiiAPI, IpcRequest, WidgetControlType};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

static ENGINE_CANCEL: Mutex<Option<tokio::sync::oneshot::Sender<()>>> = Mutex::new(None);

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
fn op_update_widget_property(
    state: &mut OpState,
    #[string] widget: String,
    #[string] name: String,
    #[string] value: String,
) {
    if let Some(Some(host)) = state.try_borrow::<Option<Arc<dyn EwwiiAPI>>>() {
        let host = Arc::clone(host);

        std::thread::spawn(move || {
            let _ =
                host.ipc_request(IpcRequest::WidgetControl(WidgetControlType::PropertyUpdate {
                    widget,
                    prop: name,
                    value,
                }));
        });
    }
}

#[op2(fast)]
fn op_widget_add_class(state: &mut OpState, #[string] widget: String, #[string] class: String) {
    if let Some(Some(host)) = state.try_borrow::<Option<Arc<dyn EwwiiAPI>>>() {
        let host = Arc::clone(host);

        std::thread::spawn(move || {
            let _ = host.ipc_request(IpcRequest::WidgetControl(WidgetControlType::AddClass {
                widget,
                class,
            }));
        });
    }
}

#[op2(fast)]
fn op_widget_remove_class(state: &mut OpState, #[string] widget: String, #[string] class: String) {
    if let Some(Some(host)) = state.try_borrow::<Option<Arc<dyn EwwiiAPI>>>() {
        let host = Arc::clone(host);

        std::thread::spawn(move || {
            let _ = host.ipc_request(IpcRequest::WidgetControl(WidgetControlType::RemoveClass {
                widget,
                class,
            }));
        });
    }
}

deno_core::extension!(
    jscore_extension,
    ops = [
        op_register_window_json,
        op_update_widget_property,
        op_widget_add_class,
        op_widget_remove_class
    ]
);

pub static BOOTSTRAP_JS: &str = include_str!("bootstrap/bootstrap.js");
pub static AFTER_RENDER_JS: &str = include_str!("bootstrap/after_render.js");

pub struct Engine {
    widget_state: Arc<Mutex<WidgetNode>>,
    host: Option<Arc<dyn EwwiiAPI>>,
}

impl Engine {
    pub fn new() -> Self {
        Self { widget_state: Arc::new(Mutex::new(WidgetNode::Tree(vec![]))), host: None }
    }

    pub fn set_host(&mut self, host: Arc<dyn EwwiiAPI>) {
        self.host = Some(host);
    }

    pub fn start_engine(&self, user_js_code: &str, user_js_path: &str) {
        let user_js_code_clone = user_js_code.to_string();
        let user_js_path_clone = user_js_path.to_string();
        let widget_state_clone = self.widget_state.clone();
        let host_clone = self.host.clone();

        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<()>();

        if let Some(thread_tx) = ENGINE_CANCEL.lock().unwrap().take() {
            let _ = thread_tx.send(());
        }

        let (thread_tx, thread_rx) = tokio::sync::oneshot::channel::<()>();
        *ENGINE_CANCEL.lock().unwrap() = Some(thread_tx);

        std::thread::spawn(move || {
            let user_js_code = user_js_code_clone;
            let user_js_path = user_js_path_clone;

            let mut runtime_opts = RuntimeOptions::default();
            runtime_opts.extensions = vec![
                jscore_extension::init(),
                jscore_timers::init(),
                jscore_fetch::init(),
                jscore_cmd::init(),
                jscore_fs::init(),
            ];
            runtime_opts.module_loader = Some(Rc::new(CustomResolver::new()));
            runtime_opts.create_params =
                Some(v8::Isolate::create_params().heap_limits(0, 128 * 1024 * 1024));

            let mut runtime = JsRuntime::new(runtime_opts);
            runtime.execute_script("__bootstrap.js", BOOTSTRAP_JS).unwrap();
            runtime.execute_script("__runtime.js", AFTER_RENDER_JS).unwrap();

            let op_state = runtime.op_state();
            op_state.borrow_mut().put(widget_state_clone.clone());
            op_state.borrow_mut().put(host_clone.clone());

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
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
                    eprintln!("{err}");
                    return;
                }

                let _ = ready_tx.send(());

                if let Err(err) = evaluation_result.await {
                    eprintln!("Uncaught JavaScript Exception: {}", err);
                    return;
                }

                // maybe wait for windows to be initialized?
                trigger_after_render_lifecycle(&mut runtime, module_id).await;

                tokio::select! {
                    _ = runtime.run_event_loop(Default::default()) => {}
                    _ = thread_rx => { return; }
                }
            });
        });

        let _ = ready_rx.recv();
    }

    pub fn get_widgetnode(&self) -> Arc<Mutex<WidgetNode>> {
        self.widget_state.clone()
    }
}

async fn trigger_after_render_lifecycle(runtime: &mut JsRuntime, module_id: deno_core::ModuleId) {
    let module_namespace =
        runtime.get_module_namespace(module_id).expect("Failed to get module namespace");
    let global_ctx = runtime.main_context();
    let isolate = runtime.v8_isolate();

    let global_handles = {
        v8::scope_with_context!(scope, &mut *isolate, global_ctx);
        let module_obj = v8::Local::new(&mut *scope, module_namespace);

        let key = v8::String::new(&mut *scope, "after_render").unwrap();
        if let Some(exported_value) = module_obj.get(&mut *scope, key.into()) {
            if exported_value.is_function() {
                let function: v8::Local<v8::Function> = exported_value.try_into().unwrap();

                let script_source = v8::String::new(&mut *scope, "new WidgetAPI()").unwrap();
                let script = v8::Script::compile(&mut *scope, script_source, None).unwrap();
                let api_instance = script.run(&mut *scope).unwrap();

                Some((
                    v8::Global::new(&mut *scope, function),
                    v8::Global::new(&mut *scope, api_instance),
                ))
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some((global_function, global_api_instance)) = global_handles {
        let call_future = runtime.call_with_args(&global_function, &[global_api_instance]);

        match call_future.await {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Runtime Error occurred during after_render script execution: {}", err)
            }
        }
    }
}
