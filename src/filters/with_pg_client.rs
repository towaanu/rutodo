use deadpool_postgres::{Client, Pool};
use std::convert::Infallible;
use warp::Filter;

pub fn with_pg_client(pool: Pool) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: Pool| async move {
            let client = pool.get().await.unwrap();
            Ok::<Client, Infallible>(client)
        })
}
