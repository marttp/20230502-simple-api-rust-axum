mod model;
mod handler;
mod response;
mod route;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);

    // Layer will be applied from bottom to top
    let app = create_router()
        .layer(cors);

    println!("🚀 Server started successfully on port 8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
