use warp::Filter;

mod app_logger;

#[tokio::main]
async fn main() {
    app_logger::init_logger();
    let index = warp::path::end().map(|| "Hello world !");

    log::info!("Server running on port {}", 3030);
    warp::serve(index).run(([0, 0, 0, 0], 3030)).await;
}
