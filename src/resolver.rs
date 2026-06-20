use deno_core::error::ModuleLoaderError;
use deno_core::{
    FsModuleLoader, ModuleLoader, ModuleLoadResponse, ModuleSpecifier, ModuleSourceCode, ModuleType,
};

pub struct CustomResolver {
    fs_loader: FsModuleLoader,
}

impl CustomResolver {
    pub fn new() -> Self {
        Self {
            fs_loader: FsModuleLoader,
        }
    }
}

impl ModuleLoader for CustomResolver {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: deno_core::ResolutionKind,
    ) -> Result<ModuleSpecifier, ModuleLoaderError> {
        match specifier {
            "ewwii/widgets" => {
                return Ok(ModuleSpecifier::parse("embed://ewwii/widgets.js").unwrap());
            }
            "ewwii/tools" => {
                return Ok(ModuleSpecifier::parse("embed://ewwii/tools.js").unwrap());
            }
            _ => {}
        }

        if specifier.starts_with("ext:") {
            return Ok(ModuleSpecifier::parse(specifier).unwrap());
        }

        self.fs_loader.resolve(specifier, referrer, kind)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<&deno_core::ModuleLoadReferrer>,
        options: deno_core::ModuleLoadOptions,
    ) -> ModuleLoadResponse {
        const WIDGETS_MODULE_SRC: &str = include_str!("bootstrap/widgets.js");
        const TOOLS_MODULE_SRC: &str = include_str!("bootstrap/tools.js");

        match module_specifier.as_str() {
            "embed://ewwii/widgets.js" => {
                return register_module(module_specifier, WIDGETS_MODULE_SRC);
            }
            "embed://ewwii/tools.js" => {
                return register_module(module_specifier, TOOLS_MODULE_SRC);
            }
            _ => {}
        }

        self.fs_loader.load(module_specifier, maybe_referrer, options)
    }
}

fn register_module(module_specifier: &ModuleSpecifier, module: &str) -> ModuleLoadResponse {
    return ModuleLoadResponse::Sync(Ok(deno_core::ModuleSource::new(
        ModuleType::JavaScript,
        ModuleSourceCode::String(module.to_string().into()),
        module_specifier,
        None,
    )));
}
