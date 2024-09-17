mod api;
mod utils;

use crate::api::client_struct::CandleRequest;
use crate::api::response_structs::CandlesSnapshotResponse;
use crate::utils::outer::to_timestamp_millis;
use std::collections::HashMap;
use crate::api::client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();

    let coin = "BTC";
    let interval = "1d";
    let start_time = to_timestamp_millis("2024-09-01T12:34:56+00:00");
    let end_time = to_timestamp_millis("2024-09-15T13:00:00Z");

    let mut req = HashMap::new();
    req.insert("coin".to_string(), serde_json::json!(coin));
    req.insert("interval".to_string(), serde_json::json!(interval));
    req.insert("startTime".to_string(), serde_json::json!(start_time));
    req.insert("endTime".to_string(), serde_json::json!(end_time));

    let candle_request = CandleRequest {
        request_type: "candleSnapshot".to_string(),
        req,
    };

    let candles: Vec<CandlesSnapshotResponse> = client.post("info", &candle_request).await?;
    for candle in candles {
        println!("{:?}", candle);
    }



    Ok(())
}
