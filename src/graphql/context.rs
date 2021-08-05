pub struct Context {
    pub pg_pool: deadpool_postgres::Pool,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn get_pg_client(&self) -> deadpool_postgres::Client {
        self.pg_pool.get().await.unwrap()
    }
}
