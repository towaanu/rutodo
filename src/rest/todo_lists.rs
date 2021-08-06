use crate::filters::with_pg_client::with_pg_client;
use deadpool_postgres::Pool;
use uuid::Uuid;
use warp::Filter;

pub fn routes(
    prefix: &'static str,
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path(prefix).and(
        index(pg_pool.clone())
            .or(find_todo_lists(pg_pool.clone()))
            .or(create_todo_list(pg_pool.clone()))
            .or(delete_todo_list(pg_pool)),
    )
}

pub fn index(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::list_todo_lists)
}

pub fn find_todo_lists(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(Uuid)
        .and(warp::get())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::todo_list_by_id)
}

pub fn create_todo_list(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::create_todo_list)
}

pub fn delete_todo_list(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(Uuid)
        .and(warp::delete())
        .and(with_pg_client(pg_pool))
        .and_then(handlers::delete_todo_list)
}

mod handlers {
    use crate::todos::{models::NewTodoList, todos};
    use deadpool_postgres::Client;
    use uuid::Uuid;
    use warp::http::StatusCode;

    pub async fn list_todo_lists(pg_client: Client) -> Result<impl warp::Reply, warp::Rejection> {
        let todo_lists = todos::find_todo_lists(&pg_client).await.unwrap();

        Ok(warp::reply::json(&todo_lists))
    }

    pub async fn todo_list_by_id(
        id: Uuid,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let todo_list = todos::find_todo_list_by_id(&pg_client, &id).await.unwrap();
        Ok(warp::reply::json(&todo_list))
    }

    pub async fn create_todo_list(
        new_todo_list: NewTodoList,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let created_todo_list = todos::create_todo_list(&pg_client, &new_todo_list)
            .await
            .unwrap();
        Ok(warp::reply::json(&created_todo_list))
    }

    pub async fn delete_todo_list(
        id: Uuid,
        pg_client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let nb_row = todos::delete_todo_list(&pg_client, &id).await.unwrap();
        if nb_row == 1 {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
    }
}
