use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ::app::app::App;
use app::{config::Settings, db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Cannot load settings");
    let pool = db::create_db_pool(settings.clone())
        .await
        .expect("can't connect to database");
    let app = App::new(pool);
    let router = app.load_router();
    let ip_address: IpAddr = if cfg!(debug_assertions) {
        Ipv4Addr::LOCALHOST.into()
    } else {
        Ipv4Addr::UNSPECIFIED.into()
    };
    let socket_address = SocketAddr::new(ip_address, settings.port);
    let listener = TcpListener::bind(&socket_address).await.unwrap();
    println!("listening on {}", socket_address);
    axum::serve(listener, router)
        .with_graceful_shutdown(App::shutdown_signal())
        .await
        .unwrap();
}
