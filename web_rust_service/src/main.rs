mod config;
mod models;
mod register;
mod routes;
mod services;
mod utils;
mod test;

use crate::config::AppConfig;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    // 加载配置
    let config = AppConfig::load().expect("Error loading configuration");
    // 初始化日志
    utils::init_logger(&config.log.level);
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = routes::create_routes_and_init_app_state(&config).await;
    debug!("{:?}", config);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config.server_addr())
        .await
        .unwrap();
    info!("server started at http://{}", config.server_addr());
    axum::serve(listener, app).await.unwrap();
}
