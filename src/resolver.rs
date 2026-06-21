use deno_core::error::ModuleLoaderError;
use deno_core::{
    FsModuleLoader, ModuleLoader, ModuleLoadResponse, ModuleSpecifier, ModuleSourceCode, ModuleType,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::process::Command;
use std::path::PathBuf;

pub struct CustomResolver {
    fs_loader: FsModuleLoader,
    script_dir: Option<PathBuf>,
}

impl CustomResolver {
    pub fn new() -> Self {
        Self {
            fs_loader: FsModuleLoader,
            script_dir: std::env::current_dir().ok()
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

        if specifier.starts_with("esbuild:") {
            let installed = Command::new("which")
                .arg("esbuild")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map_err(|e| ModuleLoaderError::generic(format!("which command failed: {e}")))?;

            if !installed.success() {
                return Err(ModuleLoaderError::generic("esbuild must be installed and in $PATH"));
            }

            let library = specifier.strip_prefix("esbuild:")
                .ok_or(ModuleLoaderError::generic("failed to extract library name"))?;
            let cache = cache_path(&self.script_dir, library);

            if !cache.exists() {
                Command::new("esbuild")
                    .arg(library)
                    .arg("--bundle")
                    .arg("--format=esm")
                    .arg("--platform=browser")
                    .arg(format!("--outfile={}", cache.display()))
                    .status()
                    .map_err(|e| ModuleLoaderError::generic(format!("esbuild failed: {e}")))?;
            }

            let cache_full = cache.canonicalize()
                .map_err(|e| ModuleLoaderError::generic(format!("failed to canonicalize path: {e}")))?;

            return ModuleSpecifier::from_file_path(cache_full)
                .map_err(|_| ModuleLoaderError::generic("failed to convert cache path to file URL"));
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

fn cache_path(script_dir: &Option<PathBuf>, package: &str) -> std::path::PathBuf {
    let mut hasher = DefaultHasher::new();
    package.hash(&mut hasher);
    let hash = hasher.finish();
    let temp = std::env::temp_dir();
    let base = script_dir.as_deref().unwrap_or(&temp);
    base.join(".cache")
        .join("jscore")
        .join(format!("{:x}.js", hash))
}
