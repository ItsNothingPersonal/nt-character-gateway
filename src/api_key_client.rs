extern crate redis;

use hyper::StatusCode;
use redis::{Commands, Connection};

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
}
