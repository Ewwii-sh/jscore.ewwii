use super::DenoOpError;
use deno_core::error::CoreError;
use deno_core::op2;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;
use tokio::sync::mpsc;

static NEXT_ID: AtomicU32 = AtomicU32::new(1);
static LISTENERS: OnceLock<Mutex<HashMap<u32, mpsc::Receiver<String>>>> = OnceLock::new();

fn get_listeners() -> &'static Mutex<HashMap<u32, mpsc::Receiver<String>>> {
    LISTENERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn to_core_error<E: std::fmt::Display>(e: E) -> CoreError {
    DenoOpError::msg(e.to_string()).into()
}

#[op2]
#[string]
async fn op_run(#[string] cmd: String) -> Result<String, CoreError> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .await
        .map_err(to_core_error)?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[op2]
#[smi]
async fn op_run_listen(#[string] cmd: String) -> Result<u32, CoreError> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
        .map_err(to_core_error)?;

    let mut stdout_lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut stderr_lines = BufReader::new(child.stderr.take().unwrap()).lines();

    let (tx, rx) = mpsc::channel::<String>(100);
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

    get_listeners().lock().unwrap().insert(id, rx);

    tokio::spawn(async move {
        loop {
            tokio::select! {
                maybe_line = stdout_lines.next_line() => {
                    match maybe_line {
                        Ok(Some(line)) => {
                            let val = line.trim().to_string();
                            if tx.send(val).await.is_err() {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                maybe_err = stderr_lines.next_line() => {
                    if let Ok(Some(line)) = maybe_err {
                        eprintln!("stream_cmd_lines stderr: {}", line);
                    }
                }
            }
        }
    });

    Ok(id)
}

#[op2]
#[string]
async fn op_event_update(#[smi] handle: u32) -> Result<Option<String>, CoreError> {
    let mut rx = {
        get_listeners()
            .lock()
            .unwrap()
            .remove(&handle)
            .ok_or_else(|| DenoOpError::msg("Invalid or expired stream handle"))?
    };

    let result = rx.recv().await;

    if result.is_some() {
        get_listeners().lock().unwrap().insert(handle, rx);
    }

    Ok(result)
}

#[op2]
async fn op_event_cleanup(#[smi] handle: u32) {
    if let Some(mut rx) = get_listeners().lock().unwrap().remove(&handle) {
        rx.close(); 
    }
}

deno_core::extension!(
    jscore_cmd, 
    ops = [op_run, op_run_listen, op_event_update, op_event_cleanup]
);
