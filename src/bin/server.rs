use axum::extract::State;
use axum::routing::get;
use dioxus_fullstack::prelude::*;
use issue_tracker_rust::*;
#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    let _ = PostServerData::register_explicit();
    let _ = GetServerData::register_explicit();

    axum::Server::bind(&addr)
        .serve(
            axum::Router::new()
                .register_server_fns("")
                .into_make_service(),
        )
        .await
        .unwrap();
}
