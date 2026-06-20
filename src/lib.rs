mod resolver;
mod engine;
mod convert;

use ewwii_plugin_api::{
    auto_plugin, PluginInfo,
    ConfigInfo, ParseFn,
    ParseFnExt
};

pub static MAIN_FILE: &str = "ewwii.js";

auto_plugin!(
    DummyStructure,
    PluginInfo::new("ewwii.lang.jscore", "1.0.0"),
    host,
    {
        host.log("Jscore says Hello!");

        host.register_config_engine(
            ConfigInfo {
                extension: "js",
                main_file: MAIN_FILE,

            },
            ParseFn::new(|source, path| {
                // source (&str) - source code of ewwii.js
                // path (&str) - path to ewwii.js

                let engine = engine::Engine::new();
                engine.start_engine(source, path);
            
                let wnode_arc = engine.get_widgetnode();
                let guard = wnode_arc.lock().unwrap();
                let wnode = guard.clone();

                Ok(wnode)
            }
        ));
    }
);

