pub mod todo_lists;
pub mod version;
use deadpool_postgres::Pool;

use warp::Filter;

pub fn routes(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(version::routes("version"))
        .or(todo_lists::routes("todo-lists", pg_pool.clone()))
}
