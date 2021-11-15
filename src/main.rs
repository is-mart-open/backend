use std::env;

use async_std::task;
use dotenv::dotenv;
use eyre::Result;
use sqlx::Postgres;
use tide::security::{CorsMiddleware};
use tide_sqlx::SQLxMiddleware;

mod insert_mart;
mod messages;
mod response_struct;
mod router;

fn main() -> Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port: u16 = env::var("PORT").unwrap_or("4000".to_string()).parse()?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not served");
    task::block_on(start(&host, port, &database_url))?;
    Ok(())
}

async fn start(host: &str, port: u16, database_url: &str) -> Result<()> {
    tide::log::start();

    let mut app = tide::new();
    app.with(SQLxMiddleware::<Postgres>::new(database_url).await?);
    app.with(CorsMiddleware::new().allow_origin(vec!["http://localhost:3000", "http://10.0.1.4:3000", "https://is-mart-open.btry.dev"]));

    app.at("/marts").get(router::get_mart_list);
    app.at("/marts/:name").get(router::get_mart_info);
    app.at("/location/:lat/:lon").get(router::location);
    app.at("/health").get(router::health);

    app.at("/insert/:mart").get(router::insert);

    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}
