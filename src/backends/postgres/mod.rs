mod users;

use std::fmt::{Debug, Formatter};
use sqlx::{Database, Describe, Either, Error, Execute, Executor, Pool, Postgres};
use std::ops::Deref;
use std::sync::{Arc, LazyLock, LockResult, RwLock};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use tokio::runtime::Handle;
pub use users::*;
mod roles;
pub use roles::*;
mod policies;
mod tokens;

pub use policies::*;


#[derive(Debug)]
pub struct PostgresBackend {
    pool: RwLock<Option<Arc<Pool<Postgres>>>>,
}

impl PostgresBackend {
    async fn get(&self) -> Arc<Pool<Postgres>> {
        // Check if the pool is already initialized
        if let Some(pool) = self.pool.read().unwrap().as_ref() {
            return Arc::clone(pool);
        }

        // Load environment variables
        let _ = dotenv::dotenv().ok();
        let usr = std::env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME must be set");
        let pw = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let db = std::env::var("POSTGRES_DBNAME").expect("POSTGRES_DBNAME must be set");
        let host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");

        // Create a new connection pool
        let pool = Pool::<Postgres>::connect(&format!("postgres://{usr}:{pw}@{host}/{db}"))
            .await
            .expect("Failed to connect to database!");
        let pool = Arc::new(pool);

        // Store the pool in the RwLock
        {
            let mut p = self.pool.write().unwrap();
            *p = Some(Arc::clone(&pool));
        }

        pool
    }

}

pub const POSTGRES_DB: PostgresBackend = PostgresBackend { pool: RwLock::new(None) };
// pub const POSTGRES_DB: LazyLock<Pool<Postgres>> = LazyLock::new(|| {
//     let rt = tokio::runtime::Runtime::new().expect("Failed to build runtime!");
//     rt.block_on(async {
//         let _ = dotenv::dotenv().ok();
//         let usr: String = std::env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME must be set");
//         let pw: String = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
//         let db: String = std::env::var("POSTGRES_DBNAME").expect("POSTGRES_DBNAME must be set");
//         let host: String = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
//
//         let pool = Pool::<Postgres>::connect(&format!("postgres://{usr}:{pw}@{host}/{db}")).await.expect("Failed to connect to database!");
//
//         pool
//     })
// });
// pub static POSTGRES_DB: LazyLock<Pool<Postgres>> = LazyLock::new(|| {
//     // tokio::runtime::Runtime::new().unwrap().block_on(async {
//
//     futures::executor::block_on(async {
//         let _ = dotenv::dotenv().ok();
//         let usr: String = std::env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME must be set");
//         let pw: String = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
//         let db: String = std::env::var("POSTGRES_DBNAME").expect("POSTGRES_DBNAME must be set");
//         let host: String = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
//
//         let pool = Pool::<Postgres>::connect(&format!("postgres://{usr}:{pw}@{host}/{db}")).await.expect("Failed to connect to database!");
//
//         pool
//     })
// });
