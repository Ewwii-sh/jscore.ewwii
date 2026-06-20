use deno_core::op2;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio::sync::oneshot;

static TIMERS: Mutex<Option<HashMap<u32, oneshot::Sender<()>>>> = Mutex::new(None);

fn get_timers() -> std::sync::MutexGuard<'static, Option<HashMap<u32, oneshot::Sender<()>>>> {
    let mut guard = TIMERS.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    guard
}

#[op2]
async fn op_sleep(#[smi] id: u32, delay_ms: f64) {
    let (tx, rx) = oneshot::channel::<()>();
    get_timers().as_mut().unwrap().insert(id, tx);
    
    tokio::select! {
        _ = sleep(Duration::from_millis(delay_ms as u64)) => {}
        _ = rx => {}
    }
    
    get_timers().as_mut().unwrap().remove(&id);
}

#[op2(fast)]
fn op_cancel_timer(#[smi] id: u32) {
    if let Some(tx) = get_timers().as_mut().unwrap().remove(&id) {
        let _ = tx.send(());
    }
}

deno_core::extension!(
    jscore_timers,
    ops = [op_sleep, op_cancel_timer]
);
