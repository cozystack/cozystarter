use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ::app::app::App;
use app::db;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load config

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/app".to_string());
    let pool = db::create_db_pool(db_connection_str).await;
    let app = App::new(pool);
    let router = app.load_router();
    let ip_address: IpAddr = if cfg!(debug_assertions) {
        Ipv4Addr::LOCALHOST.into()
    } else {
        Ipv4Addr::UNSPECIFIED.into()
    };
    let socket_address = SocketAddr::new(ip_address, 3000);
    let listener = TcpListener::bind(&socket_address).await.unwrap();
    println!("listening on {}", socket_address);
    axum::serve(listener, router)
        .with_graceful_shutdown(App::shutdown_signal())
        .await
        .unwrap();
}
