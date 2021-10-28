use sqlx::{Acquire, Postgres};
use tide::{Body, Request};
use tide_sqlx::SQLxRequestExt;
use urlencoding::decode;

use crate::response_struct::{Info, Location, Search};

pub async fn search(req: Request<()>) -> tide::Result<Body> {
    let mart = req.param("mart")?;
    let keyword = decode(req.param("keyword")?)?;

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"SELECT mart_name
        FROM   mart
        WHERE  mart_type = $1 AND mart_name LIKE $2;"#,
        mart,
        format!("%{}%", keyword))
        .fetch_all(pg_conn.acquire().await?)
        .await?;

    Body::from_json(&Search {
        result: row.iter().map(|rec| rec.mart_name.clone()).collect()
    })
}

pub async fn info(req: Request<()>) -> tide::Result<Body> {
    let mart = req.param("mart")?;
    let name = decode(req.param("name")?)?.to_string();
    
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"SELECT mart_name, start_time, end_time, next_holiday
        FROM   mart
        WHERE  mart_type = $1 AND mart_name LIKE $2;"#,
        mart,
        name)
        .fetch_one(pg_conn.acquire().await?)
        .await?;

    Body::from_json(&Info {
        name: row.mart_name,
        start_time: row.start_time.to_string(),
        end_time: row.end_time.to_string(),
        next_holiday: match row.next_holiday {
            Some(date) => Some(date.to_string()),
            None => None
        },
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
                next_holiday: Some("2021/10/27".to_string()),
                distance: Some(1254)
            }
        ]
    })
}