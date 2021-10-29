use serde::{Deserialize, Serialize};

use crate::messages::Messages;

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    pub result: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct InfoResponse {
    pub base_date: String,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub next_holiday: Option<String>,
    pub distance: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct LocationResponse {
    pub result: Vec<InfoResponse>,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: Messages
}