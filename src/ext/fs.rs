use super::DenoOpError;
use deno_core::error::CoreError;
use deno_core::op2;
use serde::Serialize;
use std::os::unix::fs::PermissionsExt;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[op2]
#[string]
async fn op_fs_read(#[string] path: String) -> Result<String, CoreError> {
    fs::read_to_string(&path).await.map_err(|e| DenoOpError::map(e).0)
}

#[op2]
async fn op_fs_write(
    #[string] path: String,
    #[string] content: String,
    append: bool,
) -> Result<(), CoreError> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create(true);

    if append {
        options.append(true);
    } else {
        options.truncate(true);
    }

    let mut file = options.open(&path).await.map_err(|e| DenoOpError::map(e))?;

    file.write_all(content.as_bytes()).await.map_err(|e| DenoOpError::map(e))?;

    Ok(())
}

#[op2]
async fn op_fs_remove(#[string] path: String) -> Result<(), CoreError> {
    let meta = fs::metadata(&path).await.map_err(|e| DenoOpError::map(e))?;

    if meta.is_dir() {
        fs::remove_dir(&path).await.map_err(|e| DenoOpError::map(e))?;
    } else {
        fs::remove_file(&path).await.map_err(|e| DenoOpError::map(e))?;
    }

    Ok(())
}

#[op2]
async fn op_fs_exists(#[string] path: String) -> Result<bool, CoreError> {
    Ok(fs::try_exists(&path).await.unwrap_or(false))
}

#[op2]
async fn op_fs_mkdir(#[string] path: String) -> Result<(), CoreError> {
    fs::create_dir_all(&path).await.map_err(|e| DenoOpError::map(e).0)
}

#[op2]
#[string]
async fn op_fs_readdir(#[string] path: String) -> Result<String, CoreError> {
    let mut entries = fs::read_dir(&path).await.map_err(|e| DenoOpError::map(e))?;
    let mut items: Vec<String> = Vec::new();

    while let Some(entry) = entries.next_entry().await.map_err(|e| DenoOpError::map(e))? {
        let filename = entry.file_name();
        items.push(filename.to_string_lossy().to_string());
    }

    serde_json::to_string(&items).map_err(|e| DenoOpError::map(e).0)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StatResult {
    size: u64,
    is_file: bool,
    is_dir: bool,
    created: Option<u64>,
    modified: Option<u64>,
    accessed: Option<u64>,
    mode: u32,
}

#[op2]
#[string]
async fn op_fs_stat(#[string] path: String) -> Result<String, CoreError> {
    let meta = tokio::fs::metadata(&path).await.map_err(|e| DenoOpError::map(e))?;

    let size = meta.len();
    let is_file = meta.is_file();
    let is_dir = meta.is_dir();

    let to_ms = |time: std::io::Result<std::time::SystemTime>| {
        time.ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
    };

    let created = to_ms(meta.created());
    let modified = to_ms(meta.modified());
    let accessed = to_ms(meta.accessed());

    let mode = meta.permissions().mode() & 0o777;

    let stat = StatResult { size, is_file, is_dir, created, modified, accessed, mode };

    let json_string = serde_json::to_string(&stat).map_err(|e| DenoOpError::map(e))?;
    Ok(json_string)
}

#[op2]
async fn op_fs_copy(#[string] src: String, #[string] dest: String) -> Result<(), CoreError> {
    fs::copy(&src, &dest).await.map_err(|e| DenoOpError::map(e))?;

    Ok(())
}

#[op2]
async fn op_fs_move(#[string] src: String, #[string] dest: String) -> Result<(), CoreError> {
    fs::rename(&src, &dest).await.map_err(|e| DenoOpError::map(e))?;

    Ok(())
}

deno_core::extension!(
    jscore_fs,
    ops = [
        op_fs_read,
        op_fs_write,
        op_fs_remove,
        op_fs_exists,
        op_fs_mkdir,
        op_fs_readdir,
        op_fs_stat,
        op_fs_copy,
        op_fs_move
    ]
);
