use serde_json;

#[derive(Serialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum Response {
    Success(serde_json::Value),
    Error(String),
}
