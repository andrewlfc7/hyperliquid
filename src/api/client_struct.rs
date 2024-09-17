use serde::Serialize;
use std::collections::HashMap;
use serde_json;

#[derive(Serialize)]
pub struct CandleRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub req: HashMap<String, serde_json::Value>,
}

