mod api;
mod models;
mod prisma;

use api::create_route;
use axum::{response::Html, routing::get, Router};
use prisma::{get_prisma_client, Database};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let prisma_client: Database = get_prisma_client().await;

    // build our application with a route
    let app: Router = Router::new()
        .route("/", get(handler))
        .nest("/api", create_route())
        .layer(prisma_client);

    // run it
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
