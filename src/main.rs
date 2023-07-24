use autometrics::objectives::{Objective, ObjectiveLatency, ObjectivePercentile};
use autometrics::{autometrics, prometheus_exporter};
use axum::extract::Path;
use axum::routing::{delete, post, put};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99_9);

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_all_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route(
            "/metrics",
            get(|| async { prometheus_exporter::encode_http_response() }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[autometrics(objective = API_SLO)]
async fn get_all_users() -> impl IntoResponse {
    vec![]
}

#[autometrics(objective = API_SLO)]
async fn get_user_by_id(Path(id): Path<i64>) -> impl IntoResponse {
    // Find user by id
    let user = User {
        name: "foo".to_string(),
        email: "foo@bar.xyz".to_string(),
    };
    // do something with User object

    user
}

#[autometrics(objective = API_SLO)]
async fn create_user(Json(user): Json<User>) -> User {
    let user = User {
        name: user.name,
        email: user.email,
    };
    // do something with User object

    user
}

#[autometrics(objective = API_SLO)]
async fn update_user(Path(id): Path<i64>) -> i64 {
    0
}

#[autometrics(objective = API_SLO)]
async fn delete_user(Path(id): Path<i64>) -> i64 {
    0
}
