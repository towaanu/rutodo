use warp::Filter;

mod app_logger;
mod config;
mod graphql;

#[tokio::main]
async fn main() {
    app_logger::init_logger();
    let app_config = config::get_config();
    let index = warp::path::end().map(|| "Hello world !");

    let graphql_state = warp::any()
        .map( || graphql::context::Context{});

    let graphql_filter =
        juniper_warp::make_graphql_filter(graphql::schema::new_schema(), graphql_state.boxed());

    let graphiql_api = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::playground_filter("/graphql", None));

    let graphql_api = warp::path("graphql").and(graphql_filter);

    let api = index
        .or(graphql_api)
        .or(graphiql_api)
        .with(warp::log("rutodo::api"));

    log::info!("Server running on port {}", app_config.port);
    warp::serve(api).run(([0, 0, 0, 0], app_config.port)).await;
}
