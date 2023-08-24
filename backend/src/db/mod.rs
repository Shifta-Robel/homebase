use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use sea_orm::error::DbErr;
use std::time::Duration;

pub async fn connect(path: &str) -> Result<DatabaseConnection, DbErr> {
    // let url = "sqlite://./src/db/quicklinks.sqlite";

    // let mut opt = ConnectOptions::new(url);
    let mut opt = ConnectOptions::new(path);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        // .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;
    Ok(db)
}

pub async fn disconnect(db: DatabaseConnection) -> Result<(), DbErr> {
    db.close().await?;
    Ok(())
}
