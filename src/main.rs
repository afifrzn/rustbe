use axum::{
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    if payload.username == "admin" && payload.password == "1234" {
        Json(LoginResponse {
            success: true,
            message: "Login berhasil!".to_string(),
        })
    } else {
        Json(LoginResponse {
            success: false,
            message: "Username atau password salah".to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(login_handler))
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
