use deno_core::{op2, v8, JsRuntime, RuntimeOptions, FsModuleLoader};
use std::rc::Rc;

#[op2(fast)]
fn op_print_to_host(#[string] msg: &str) {
    println!("Host received: {}", msg);
}

deno_core::extension!(
    jscore_extension,
    ops = [op_print_to_host]
);

pub static BOOTSTRAP_JS: &str = r#"
    globalThis.op_print_to_host = Deno.core.ops.op_print_to_host;
"#;

fn main() {
    let mut runtime_opts = RuntimeOptions::default();
    runtime_opts.extensions = vec![jscore_extension::init()];
    runtime_opts.module_loader = Some(Rc::new(FsModuleLoader));
    runtime_opts.create_params = Some(
        v8::Isolate::create_params()
            .heap_limits(0, 128 * 1024 * 1024)
    );

    let mut runtime = JsRuntime::new(runtime_opts);
    runtime.execute_script("__bootstrap.js", BOOTSTRAP_JS).unwrap();

    let user_js_code = r#"
        op_print_to_host("Hello from inside JavaScript directly!");
        console.log("Deno is running this npm package seamlessly!");
    "#;

    futures::executor::block_on(async {
        let module_specifier = deno_core::ModuleSpecifier::parse("file:///main.js").unwrap();

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
