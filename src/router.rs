use tide::{Body, Request};
use urlencoding::decode;

use crate::response_struct::{Info, Location, Search};

pub async fn search(req: Request<()>) -> tide::Result<Body> {
    let mart = req.param("mart")?;
    let keyword = decode(req.param("keyword")?)?;
    println!("{}, {}", mart, keyword);
    Body::from_json(&Search {
        result: vec![]
    })
}

pub async fn info(req: Request<()>) -> tide::Result<Body> {
    let mart = req.param("mart")?;
    let name = decode(req.param("name")?)?;
    println!("{}, {}", mart, name);
    Body::from_json(&Info {
        name: format!("{} {}", mart, name),
        start_time: "10:00:00".to_string(),
        end_time: "22:00:00".to_string(),
        next_holiday: "2021/10/27".to_string(),
        distance: None
    })
}

pub async fn location(req: Request<()>) -> tide::Result<Body> {
    let lat = req.param("lat")?;
    let lon = req.param("lon")?;
    Body::from_json(&Location {
        result: vec![
            Info {
                name: "이마트 가든5점".to_string(),
                start_time: "10:00:00".to_string(),
                end_time: "22:00:00".to_string(),
                next_holiday: "2021/10/27".to_string(),
                distance: Some(1254)
            }
        ]
    })
}