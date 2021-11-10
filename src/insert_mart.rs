use serde_json::Value;
use sqlx::{Acquire, Postgres, types::time::Date};
use surf::http::mime;
use tide::{Request, log::info};
use tide_sqlx::SQLxRequestExt;

pub async fn insert_emart(req: &Request<()>) -> eyre::Result<(), surf::Error> {
    let response = surf::post("https://store.emart.com/branch/searchList.do")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.1 Safari/605.1.15")
        .body("srchMode=jijum&year=2021&month=11&jMode=true&strConfirmYN=N&searchType=EM&keyword=".to_string())
        .content_type(mime::FORM)
        .recv_json::<Value>()
        .await?;

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let mut transaction = pg_conn.begin().await?;
    
    for data in response["dataList"].as_array().unwrap() {
        sqlx::query(&format!(
            "INSERT INTO mart (base_date, mart_type, mart_type_name, mart_name, loc, start_time, end_time, next_holiday)
            VALUES ('{today}', 'emart', '이마트', '{name}', ST_GeomFromText('POINT({y} {x})', 4326), '{open}', '{close}', '{holiday}')
            ON CONFLICT (mart_type, mart_name) 
            DO 
            UPDATE SET base_date='{today}', loc=ST_GeomFromText('POINT({y} {x})', 4326), start_time='{open}', end_time='{close}', next_holiday='{holiday}';",
            today=Date::today(),
            name=data["NAME"].as_str().unwrap().replace("이마트 ", ""),
            y=data["MAP_Y"].as_str().unwrap(),
            x=data["MAP_X"].as_str().unwrap(),
            open=data["OPEN_SHOPPING_TIME"].as_str().unwrap(),
            close=data["CLOSE_SHOPPING_TIME"].as_str().unwrap(),
            holiday=data["HOLIDAY_DAY1_YYYYMMDD"].as_str().unwrap())
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn insert_traders(req: &Request<()>) -> eyre::Result<(), surf::Error> {
    let response = surf::post("https://store.emart.com/branch/searchList.do")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.1 Safari/605.1.15")
        .body("srchMode=jijum&year=2021&month=11&jMode=true&strConfirmYN=N&searchType=TR&keyword=".to_string())
        .content_type(mime::FORM)
        .recv_json::<Value>()
        .await?;

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let mut transaction = pg_conn.begin().await?;
    
    for data in response["dataList"].as_array().unwrap() {
        sqlx::query(&format!(
            "INSERT INTO mart (base_date, mart_type, mart_type_name, mart_name, loc, start_time, end_time, next_holiday)
            VALUES ('{today}', 'traders', '이마트 트레이더스', '{name}', ST_GeomFromText('POINT({y} {x})', 4326), '{open}', '{close}', '{holiday}')
            ON CONFLICT (mart_type, mart_name) 
            DO 
            UPDATE SET base_date='{today}', loc=ST_GeomFromText('POINT({y} {x})', 4326), start_time='{open}', end_time='{close}', next_holiday='{holiday}';",
            today=Date::today(),
            name=data["NAME"].as_str().unwrap().replace("이마트 트레이더스 ", ""),
            y=data["MAP_Y"].as_str().unwrap(),
            x=data["MAP_X"].as_str().unwrap(),
            open=data["OPEN_SHOPPING_TIME"].as_str().unwrap(),
            close=data["CLOSE_SHOPPING_TIME"].as_str().unwrap(),
            holiday=data["HOLIDAY_DAY1_YYYYMMDD"].as_str().unwrap())
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(())
}