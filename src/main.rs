use warp::Filter;

mod app_logger;
mod config;

#[tokio::main]
async fn main() {
    app_logger::init_logger();
    let app_config = config::get_config();
    let index = warp::path::end().map(|| "Hello world !");

    log::info!("Server running on port {}", app_config.port);
    warp::serve(index).run(([0, 0, 0, 0], app_config.port)).await;
}
