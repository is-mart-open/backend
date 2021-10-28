use std::env;

use async_std::task;
use dotenv::dotenv;
use eyre::Result;
use sqlx::Postgres;
use tide_sqlx::SQLxMiddleware;

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

    app.at("/search/:mart").get(router::search);
    app.at("/search/:mart/:keyword").get(router::search);
    app.at("/info/:mart/:name").get(router::info);
    app.at("/location/:lat/:lon").get(router::location);

    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}
