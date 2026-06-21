mod convert;
mod engine;
mod ext;
mod resolver;

use ewwii_plugin_api::{ConfigInfo, ParseFn, ParseFnExt, PluginInfo, auto_plugin};

pub static MAIN_FILE: &str = "ewwii.js";

auto_plugin!(DummyStructure, PluginInfo::new("ewwii.lang.jscore", "1.0.0"), host, {
    host.log("Jscore says Hello!");

    let host_clone = host.clone();
    host.register_config_engine(
        ConfigInfo { extension: "js", main_file: MAIN_FILE },
        ParseFn::new(move |source, path| {
            // source (&str) - source code of ewwii.js
            // path (&str) - path to ewwii.js

            let mut engine = engine::Engine::new();
            engine.set_host(host_clone.clone());
            engine.start_engine(source, path);

            let wnode_arc = engine.get_widgetnode();
            let guard = wnode_arc.lock().unwrap();
            let wnode = guard.clone();

            Ok(wnode)
        }),
    );
});
