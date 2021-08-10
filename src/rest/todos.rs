use crate::filters::with_pg_client::with_pg_client;
use deadpool_postgres::Pool;
use warp::Filter;

pub fn routes(
    prefix: &'static str,
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path(prefix).and(
        find_todo(pg_pool.clone())
            .or(create_todo(pg_pool.clone()))
            .or(delete_todo(pg_pool.clone()))
            .or(toggle_todo(pg_pool)),
    )
}

pub fn find_todo(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(i32)
        .and(warp::get())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::todo_by_id)
}

pub fn create_todo(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::create_todo)
}

pub fn delete_todo(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(i32)
        .and(warp::delete())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::delete_todo)
}

pub fn toggle_todo(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(i32 / "toggle")
        .and(warp::get())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::toggle_todo)
}

mod handlers {
    use crate::todos::{models::NewTodo, todos};
    use deadpool_postgres::Client;
    use warp::http::StatusCode;

    pub async fn todo_by_id(
        id: i32,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let todo = todos::find_todo_by_id(&pg_client, id).await?;
        Ok(warp::reply::json(&todo))
    }

    pub async fn create_todo(
        new_todo: NewTodo,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let created_todo = todos::create_todo(&pg_client, &new_todo).await?;
        Ok(warp::reply::json(&created_todo))
    }

    pub async fn delete_todo(
        id: i32,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let nb_row = todos::delete_todo(&pg_client, id).await?;
        if nb_row == 1 {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
    }

    pub async fn toggle_todo(
        id: i32,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let todo = todos::toggle_todo(&pg_client, id).await?;
        Ok(warp::reply::json(&todo))
    }
}
