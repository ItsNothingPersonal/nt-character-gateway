extern crate redis;

use std::env;

use hyper::StatusCode;
use redis::{Commands, Connection};

use crate::character_db::player_character::PlayerCharacter;

pub struct ApiKeyClient {
    connection: Connection,
}

impl ApiKeyClient {
    pub fn new(connection_string: String) -> ApiKeyClient {
        let client = redis::Client::open(connection_string).unwrap();
        ApiKeyClient {
            connection: client.get_connection().unwrap(),
        }
    }

    pub fn map_key(&mut self, api_key: &String) -> Result<String, StatusCode> {
        if let Ok(value) = self.connection.get::<&String, String>(api_key) {
            Ok(value)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }

    pub fn get_cached_data(&mut self, api_key: &String) -> Result<PlayerCharacter, StatusCode> {
        let cache_key = format!("cache-{}", api_key);

        if let Ok(value) = self.connection.get::<String, String>(cache_key) {
            let player_character: PlayerCharacter = serde_json::from_str(&value).unwrap();
            Ok(player_character)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }

    pub fn write_player_character_to_cache(
        &mut self,
        api_key: &String,
        player_character: &PlayerCharacter,
    ) {
        let cache_key = format!("cache-{}", api_key);
        let serialized_character = serde_json::to_string(player_character).unwrap();
        let ttl: usize = env::var("CACHE_TTL")
            .unwrap_or_else(|_| "900".to_string())
            .parse()
            .unwrap();

        let result =
            self.connection
                .set_ex::<String, String, usize>(cache_key, serialized_character, ttl);
        tracing::debug!("cache write result: {:?}", result);
    }
}
