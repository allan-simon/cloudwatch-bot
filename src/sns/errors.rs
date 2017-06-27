use serde_json;

pub(crate) type JsonResult<T> = Result<T, serde_json::Error>;
