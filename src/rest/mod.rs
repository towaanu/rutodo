pub mod rejection;
pub mod todo_lists;
pub mod version;
pub mod models;

use deadpool_postgres::Pool;

use warp::Filter;

pub fn routes(
    pg_pool: Pool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone {
    warp::any()
        .and(version::routes("version"))
        .or(todo_lists::routes("todo-lists", pg_pool))
        .recover(rejection::handle_rejection)
}
