use sqlx::{Acquire, Postgres};
use tide::{Body, Request};
use tide_sqlx::SQLxRequestExt;
use urlencoding::decode;

use crate::{insert_mart::{insert_emart, insert_traders}, messages::Messages, structs::{ErrorResponse, InfoResponse, LocationQuery, LocationResponse, SearchResponse}};

pub async fn get_mart_list(req: Request<()>) -> tide::Result<Body> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"
        SELECT mart_name
        FROM   mart
        ORDER  BY mart_name;
        "#,
    )
    .fetch_all(pg_conn.acquire().await?)
    .await?;

    if row.is_empty() {
        return Body::from_json(&ErrorResponse {
            error: Messages::EmptySearchResult,
        });
    }

    Body::from_json(&SearchResponse {
        result: row.iter().map(|rec| rec.mart_name.clone()).collect(),
    })
}

pub async fn get_mart_info(req: Request<()>) -> tide::Result<Body> {
    let name = decode(req.param("name")?)?
        .to_string()
        .split(",")
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"
        SELECT base_date, mart_name, start_time, end_time, next_holiday
        FROM   mart
        WHERE  mart_name = ANY($1);
        "#,
        &name[..]
    )
    .fetch_all(pg_conn.acquire().await?)
    .await?;

    Body::from_json(
        &row
        .into_iter()
        .map(|rec| {
            InfoResponse {
                base_date: rec.base_date.to_string(),
                name: rec.mart_name,
                start_time: rec.start_time.to_string(),
                end_time: rec.end_time.to_string(),
                next_holiday: match rec.next_holiday {
                    Some(date) => Some(date.to_string()),
                    None => None,
                },
                distance: None,
            }
        })
        .collect::<Vec<InfoResponse>>()
    )
}

pub async fn location(req: Request<()>) -> tide::Result<Body> {
    let LocationQuery { lat, lon } = req.query()?;

    let base = geoutils::Location::new(lat, lon);
    let mbr_length = 20000.0;

    let lat_diff = mbr_length
        / 2.0
        / base
            .distance_to(&geoutils::Location::new(lat + 1.0, lon))
            .unwrap()
            .meters();
    let lon_diff = mbr_length
        / 2.0
        / base
            .distance_to(&geoutils::Location::new(lat, lon + 1.0))
            .unwrap()
            .meters();

    let diagonal = format!(
        "LINESTRING({} {}, {} {})",
        lon - &lon_diff,
        lat - &lat_diff,
        lon + &lon_diff,
        lat + &lat_diff
    );

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"
        SELECT * FROM (
            SELECT base_date, mart_name, start_time, end_time, next_holiday, 
                   ST_DistanceSphere(ST_GeomFromText($1), loc) AS distance
            FROM   mart
            WHERE  ST_GeomFromText($2) ~ loc
        ) as a
        ORDER BY distance
        "#,
        format!("POINT({} {})", lon, lat),
        diagonal
    )
    .fetch_all(pg_conn.acquire().await?)
    .await?;

    Body::from_json(&LocationResponse {
        result: row
            .iter()
            .map(|rec| InfoResponse {
                base_date: rec.base_date.to_string(),
                name: rec.mart_name.clone(),
                start_time: rec.start_time.to_string(),
                end_time: rec.end_time.to_string(),
                next_holiday: match rec.next_holiday {
                    Some(date) => Some(date.to_string()),
                    None => None,
                },
                distance: Some(match rec.distance {
                    Some(dist) => dist as u64,
                    None => 0,
                }),
            })
            .collect(),
    })
}

pub async fn insert(req: Request<()>) -> tide::Result<String> {
    let mart = req.param("mart")?;
    
    Ok(match mart {
        "emart" => match insert_emart(&req).await {
            Ok(_) => "OK".to_string(),
            Err(e) => format!("{:?}", e),
        },
        "traders" => match insert_traders(&req).await {
            Ok(_) => "OK".to_string(),
            Err(e) => format!("{:?}", e),
        },
        _ => "Unsupported".to_string()
        })
}

pub async fn health(_: Request<()>) -> tide::Result<String> {
    Ok("Ok".to_string())
}