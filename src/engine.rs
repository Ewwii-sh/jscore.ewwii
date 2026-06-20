use deno_core::{op2, OpState, v8, JsRuntime, RuntimeOptions};
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use crate::resolver::CustomResolver;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

#[op2(fast)]
fn op_register_window_json(state: &mut OpState, #[string] json: &str) {
    println!("Host received: {}", json);

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

deno_core::extension!(
    jscore_extension,
    ops = [op_register_window_json]
);

pub static BOOTSTRAP_JS: &str = include_str!("bootstrap/bootstrap.js");

pub struct Engine {
    widget_state: Arc<Mutex<WidgetNode>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            widget_state: Arc::new(Mutex::new(WidgetNode::Tree(vec![]))),
        }
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

        let op_state = runtime.op_state();
        op_state.borrow_mut().put(self.widget_state.clone());

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
            }
        });
    }

    pub fn get_widgetnode(&self) -> Arc<Mutex<WidgetNode>> {
        self.widget_state.clone()
    }
}
