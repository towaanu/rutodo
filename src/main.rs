use warp::Filter;

mod app_logger;
mod config;
mod db;
mod errors;
mod filters;
mod graphql;
mod rest;
mod todos;

#[tokio::main]
async fn main() {
    app_logger::init_logger();
    let app_config = config::get_config();
    let pg_pool = db::get_pg_pool(&app_config).unwrap();
    let index = warp::path::end().map(|| "Hello world !");

    let graphql_pg_pool = pg_pool.clone();
    let graphql_state = warp::any().map(move || graphql::context::Context {
        pg_pool: graphql_pg_pool.clone(),
    });

    let graphql_filter =
        juniper_warp::make_graphql_filter(graphql::schema::new_schema(), graphql_state.boxed());

    let graphiql_api = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::playground_filter("/graphql", None));

    let graphql_api = warp::path("graphql").and(graphql_filter);

    let rest_api = warp::path("rest").and(rest::routes(pg_pool.clone()));

    let api = index
        .or(rest_api)
        .or(graphql_api)
        .or(graphiql_api)
        .with(warp::log("rutodo::api"));

    log::info!("Server running on port {}", app_config.port);
    warp::serve(api).run(([0, 0, 0, 0], app_config.port)).await;
}
