mod character_db;

use crate::character_db::player_character_client::PlayerCharacterClient;
use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use character_db::player_character::PlayerCharacter;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with the following routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/character/:sheet_key", get(character_data));

    // run our app
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

/// loading the character data from the passed in sheet
async fn character_data(
    Path(sheet_key): Path<String>,
) -> (StatusCode, Json<Option<PlayerCharacter>>) {
    tracing::debug!("Sheet Key: {:?}", sheet_key);

    let service_account_info = env::var("SERVICE_ACCOUNT_INFORMATION").unwrap_or_default();

    let player_character_client = match PlayerCharacterClient::new(service_account_info).await {
        Ok(client) => client,
        Err(error_code) => return (error_code, Json(None)),
    };

    let retrieved_character = match player_character_client.parse_data(sheet_key).await {
        Ok(data) => data,
        Err(error_code) => return (error_code, Json(None)),
    };

    // returning the result
    (StatusCode::OK, Json(Some(retrieved_character)))
}
