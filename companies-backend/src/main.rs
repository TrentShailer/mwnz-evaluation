pub mod api_error;
pub mod companies_route;
pub mod company_type;
pub mod fetch_company;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use companies_route::get_companies;
pub use company_type::Company;
pub use fetch_company::try_fetch_company;
use simplelog::{Config, TermLogger};

#[tokio::main]
async fn main() {
    TermLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Never,
    )
    .expect("Failed to init termlogger");

    let router = router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to create TCP listener");

    log::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .expect("Failed to start webserver")
}

pub fn router() -> Router {
    Router::new().route("/v1/companies/:id", get(get_companies))
}
