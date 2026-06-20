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
        if specifier == "ewwii/widgets" {
            return Ok(ModuleSpecifier::parse("embed://ewwii/widgets.js").unwrap());
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

        if module_specifier.as_str() == "embed://ewwii/widgets.js" {
            return ModuleLoadResponse::Sync(Ok(deno_core::ModuleSource::new(
                ModuleType::JavaScript,
                ModuleSourceCode::String(WIDGETS_MODULE_SRC.to_string().into()),
                module_specifier,
                None,
            )));
        }

        self.fs_loader.load(module_specifier, maybe_referrer, options)
    }
}
