mod api_key_client;
mod character_db;
mod config;

use crate::{
    api_key_client::ApiKeyClient, character_db::player_character_client::PlayerCharacterClient,
};
use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use character_db::player_character::{PlayerCharacter, PlayerCharacterUpdateInput};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with the following routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route(
            "/character/:sheet_key",
            get(character_data).put(character_update),
        );

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
    Path(api_key): Path<String>,
) -> (StatusCode, Json<Option<PlayerCharacter>>) {
    tracing::debug!("API Key: {:?}", api_key);

    let db_connection_string =
        env::var("DB_CONNECTION_STRING").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let mut api_key_client = ApiKeyClient::new(db_connection_string);

    let cached_character = api_key_client.get_cached_data(&api_key);

    if let Ok(player_character) = cached_character {
        tracing::debug!("found cached data for api key {:?}", api_key);
        return (StatusCode::OK, Json(Some(player_character)));
    }

    tracing::debug!("no cached data found, retrieving from google spreadsheets");
    let sheet_key = match api_key_client.map_key(&api_key) {
        Ok(key) => key,
        Err(err) => return (err, Json(None)),
    };

    let service_account_info = env::var("SERVICE_ACCOUNT_INFORMATION").unwrap_or_default();

    let mut player_character_client = match PlayerCharacterClient::new(service_account_info).await {
        Ok(client) => client,
        Err(error_code) => return (error_code, Json(None)),
    };

    let retrieved_character = match player_character_client.parse_data(sheet_key).await {
        Ok(data) => data,
        Err(error_code) => return (error_code, Json(None)),
    };

    api_key_client.write_player_character_to_cache(&api_key, &retrieved_character);

    // returning the result
    (StatusCode::OK, Json(Some(retrieved_character)))
}

async fn character_update(
    Path(api_key): Path<String>,
    Json(payload): Json<PlayerCharacterUpdateInput>,
) -> (StatusCode, Json<Option<i32>>) {
    tracing::debug!("API Key: {:?}", api_key);
    tracing::debug!("Payload: {:?}", payload);

    let db_connection_string =
        env::var("DB_CONNECTION_STRING").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let mut api_key_client = ApiKeyClient::new(db_connection_string);

    let sheet_key = match api_key_client.map_key(&api_key) {
        Ok(key) => key,
        Err(err) => return (err, Json(None)),
    };

    let service_account_info = env::var("SERVICE_ACCOUNT_INFORMATION").unwrap_or_default();

    let player_character_client = match PlayerCharacterClient::new(service_account_info).await {
        Ok(client) => client,
        Err(error_code) => return (error_code, Json(None)),
    };

    let updated_cells = match player_character_client.write_data(sheet_key, payload).await {
        Ok(data) => data,
        Err(error_code) => return (error_code, Json(None)),
    };

    if let Err(err_code) = api_key_client.remove_cached_data(&api_key) {
        return (err_code, Json(None));
    }

    (StatusCode::OK, Json(Some(updated_cells)))
}
