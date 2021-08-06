use warp::Filter;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("version")
        .and(index())
}

pub fn index() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .map(handlers::current_api_version)
}

mod handlers {
    pub fn current_api_version<'a>() -> &'a str {
        "0.1"
    }
}
