use serde::Serialize;
use deno_core::op2;
use std::fmt;
use deno_core::error::CoreError;

#[derive(Serialize)]
struct FetchResponse {
    status: u16,
    ok: bool,
    body: String,
}

#[op2]
#[string]
async fn op_fetch(
    #[string] url: String,
    #[string] method: String,
    #[serde] headers: Vec<(String, String)>,
    #[string] body: Option<String>,
) -> Result<String, CoreError> {
    let method = reqwest::Method::from_bytes(method.as_bytes())
        .map_err(|e| FetchOpError::map(e))?;

    let client = reqwest::Client::new();
    let mut req = client.request(method, url);
    for (key, value) in headers {
        req = req.header(key, value);
    }

    if let Some(body) = body {
        req = req.body(body);
    }

    let res = req.send().await
        .map_err(|e| FetchOpError::map(e))?;

    let status = res.status();
    let body_text = res.text().await
        .map_err(|e| FetchOpError::map(e))?;

    let fres = FetchResponse {
        status: status.as_u16(),
        ok: status.is_success(),
        body: body_text,
    };

    serde_json::to_string(&fres)
        .map_err(|e| FetchOpError::map(e).0)
}

#[derive(Debug)]
pub struct FetchOpError(pub CoreError);

impl fmt::Display for FetchOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<FetchOpError> for CoreError {
    fn from(err: FetchOpError) -> Self {
        err.0
    }
}

impl FetchOpError {
    pub fn msg(message: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::Other, 
            message.into().into_owned()
        );
        
        FetchOpError(CoreError(Box::new(deno_core::error::CoreErrorKind::Io(io_err))))
    }

    pub fn map<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, err);
        
        FetchOpError(CoreError(Box::new(deno_core::error::CoreErrorKind::Io(io_err))))
    }
}

deno_core::extension!(
    jscore_fetch,
    ops = [op_fetch]
);

