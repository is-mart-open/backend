use sqlx::{Acquire, Postgres};
use tide::{Body, Request};
use tide_sqlx::SQLxRequestExt;
use urlencoding::decode;

use crate::response_struct::{Info, Location, Search};

pub async fn search(req: Request<()>) -> tide::Result<Body> {
    let mart = req.param("mart")?;
    let keyword = decode(req.param("keyword").unwrap_or(""))?;

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"
        SELECT mart_name
        FROM   mart
        WHERE  mart_type = $1 AND mart_name LIKE $2;
        "#,
        mart,
        format!("%{}%", keyword)
    )
    .fetch_all(pg_conn.acquire().await?)
    .await?;

    Body::from_json(&Search {
        result: row.iter().map(|rec| rec.mart_name.clone()).collect(),
    })
}

pub async fn info(req: Request<()>) -> tide::Result<Body> {
    let mart = req.param("mart")?;
    let name = decode(req.param("name")?)?.to_string();

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let row = sqlx::query!(
        r#"
        SELECT mart_name, start_time, end_time, next_holiday
        FROM   mart
        WHERE  mart_type = $1 AND mart_name LIKE $2;
        "#,
        mart,
        name
    )
    .fetch_one(pg_conn.acquire().await?)
    .await?;

    Body::from_json(&Info {
        name: row.mart_name,
        start_time: row.start_time.to_string(),
        end_time: row.end_time.to_string(),
        next_holiday: match row.next_holiday {
            Some(date) => Some(date.to_string()),
            None => None,
        },
        distance: None,
    })
}

pub async fn location(req: Request<()>) -> tide::Result<Body> {
    let lat: f64 = req.param("lat")?.parse()?;
    let lon: f64 = req.param("lon")?.parse()?;

    let base = geoutils::Location::new(lat, lon);
    let mbr_length = 20000.0;

    let lat_diff = mbr_length / 2.0 / base.distance_to(&geoutils::Location::new(lat + 1.0, lon)).unwrap().meters();
    let lon_diff = mbr_length / 2.0 / base.distance_to(&geoutils::Location::new(lat, lon + 1.0)).unwrap().meters();

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
            SELECT mart_name, start_time, end_time, next_holiday, 
                  ST_DistanceSphere(ST_GeomFromText($1), loc) AS distance
            FROM   mart
            WHERE ST_GeomFromText($2) ~ loc
        ) as a
        ORDER BY distance
        LIMIT  10
        "#,
        format!("POINT({} {})", lon, lat),
        diagonal
    )
    .fetch_all(pg_conn.acquire().await?)
    .await?;

    Body::from_json(&Location {
        result: row
            .iter()
            .map(|rec| Info {
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
