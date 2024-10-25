use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CandlesSnapshotResponse {
    #[serde(rename = "t")]
    pub time_open: u64,
    #[serde(rename = "T")]
    pub time_close: u64,
    #[serde(rename = "s")]
    pub coin: String,
    #[serde(rename = "i")]
    pub candle_interval: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub vlm: String,
    #[serde(rename = "n")]
    pub num_trades: u64,
}


