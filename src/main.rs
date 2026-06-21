mod convert;
mod engine;
mod ext;
mod resolver;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !(args.len() >= 2) {
        eprintln!("A path to script must be provided.");
        std::process::exit(1);
    }

    let path_str = &args[1];
    let path = PathBuf::from(path_str);

    let user_js_code = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read file: {e}");
            std::process::exit(1);
        }
    };

    let absolute_path = fs::canonicalize(&path).unwrap();
    let engine = engine::Engine::new();
    engine.start_engine(&user_js_code, &absolute_path.to_string_lossy());

    let wnode = engine.get_widgetnode();
    if let Ok(i) = wnode.lock() {
        println!("WidgetNode: {:#?}", i);
    } else {
        eprintln!("Mutex is poisoned.");
    }
}
