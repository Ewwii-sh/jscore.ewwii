use super::DenoOpError;
use deno_core::error::CoreError;
use deno_core::op2;
use tokio::process::Command;

#[op2]
#[string]
async fn op_run(#[string] cmd: String) -> Result<String, CoreError> {
    let output =
        Command::new("sh").arg("-c").arg(&cmd).output().await.map_err(|e| DenoOpError::map(e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

deno_core::extension!(jscore_cmd, ops = [op_run]);
