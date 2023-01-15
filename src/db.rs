use std::env;
use std::time::Duration;

use sea_orm::{Database, DatabaseConnection, SqlxSqliteConnector};
use sqlx::ConnectOptions;
use sqlx::pool::PoolOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous};
use tracing::log;

pub(crate) async fn get_connection() -> DatabaseConnection {
    let db = env::var("DB").unwrap();
    if db == "pg" {
        get_connection_pg().await
    } else if db == "sqlite" {
        get_connection_sqlite().await
    } else {
        panic!("DB expected");
    }
}

async fn get_connection_sqlite() -> DatabaseConnection {
    let mut opt = SqliteConnectOptions::new()
        .filename(env::var("SQLITE_PATH").unwrap())
        .journal_mode(SqliteJournalMode::Wal)
        .locking_mode(SqliteLockingMode::Normal)
        .synchronous(SqliteSynchronous::Normal)
        .page_size(4096)
        .pragma("temp_store", "memory")
        .pragma("mmap_size", "10000")
        .pragma("cache_size", "5000")
        .create_if_missing(true)
        .shared_cache(true);

    opt.log_statements(log::LevelFilter::Off);
    opt.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(5));

    let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(
        PoolOptions::<sqlx::Sqlite>::new()
            .max_connections(4)
            .min_connections(20)
            .connect_with(opt)
            .await
            .unwrap(),
    );

    db
}

async fn get_connection_pg() -> DatabaseConnection {
    let mut opt = sea_orm::ConnectOptions::new(env::var("PG_CONNECT_URL").unwrap().to_owned());
    opt.max_connections(200)
        .min_connections(100)
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db: DatabaseConnection = Database::connect(opt).await.unwrap();
    db
}
