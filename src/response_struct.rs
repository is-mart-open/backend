use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Search {
    pub result: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub next_holiday: Option<String>,
    pub distance: Option<u16>
}

#[derive(Deserialize, Serialize)]
pub struct Location {
    pub result: Vec<Info>
}