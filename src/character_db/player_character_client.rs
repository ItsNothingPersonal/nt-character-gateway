extern crate google_sheets4 as sheets4;
extern crate yup_oauth2 as oauth2;

use crate::config::config_client::ConfigClient;

use super::player_character::{PlayerCharacter, PlayerCharacterUpdateInput};
use hyper::{client::HttpConnector, StatusCode};
use hyper_rustls::HttpsConnector;
use sheets4::{
    api::{BatchUpdateValuesRequest, ValueRange},
    Sheets,
};

pub struct PlayerCharacterClient {
    hub: Sheets<HttpsConnector<HttpConnector>>,
    sheet_config: ConfigClient,
}

impl PlayerCharacterClient {
    pub async fn new(
        service_account_information: String,
    ) -> Result<PlayerCharacterClient, StatusCode> {
        let secret = if let Ok(credentials) =
            oauth2::parse_service_account_key(&service_account_information)
        {
            tracing::debug!("parsed credentials successfully from env variable");
            credentials
        } else {
            tracing::debug!(
                "parsed credentials unsuccessfully from env variable, trying credentials.json now"
            );
            if let Ok(credentials) = oauth2::read_service_account_key("credentials.json").await {
                tracing::debug!("parsed credentials successfully from credentials.json");
                credentials
            } else {
                tracing::debug!("both parse attempts failed, giving up");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let authenticator = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .expect("failed to create authenticator");

        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            authenticator,
        );

        let sheet_config = ConfigClient::new();
        Ok(PlayerCharacterClient { hub, sheet_config })
    }

    pub async fn parse_data(&mut self, sheet_id: String) -> Result<PlayerCharacter, StatusCode> {
        match self.load_data(sheet_id).await {
            Ok(data) => Ok(PlayerCharacter::from(data)),
            Err(err) => Err(err),
        }
    }

    async fn load_data(&self, sheet_key: String) -> Result<Vec<ValueRange>, StatusCode> {
        let mut result = self.hub.spreadsheets().values_batch_get(sheet_key.as_str());

        for entry in self.sheet_config.get_field_config_sorted() {
            if entry.exclude_on_read.unwrap_or(false) {
                continue;
            }

            result = result.add_ranges(&entry.range);
        }

        let response = result.doit().await;

        match response {
            Ok(my_result) => Ok(my_result.1.value_ranges.unwrap()),
            Err(..) => Err(StatusCode::NOT_FOUND),
        }
    }

    pub async fn write_data(
        &self,
        sheet_key: String,
        character: PlayerCharacterUpdateInput,
    ) -> Result<i32, StatusCode> {
        let update_request = BatchUpdateValuesRequest {
            data: Some(character.into()),
            include_values_in_response: None,
            response_date_time_render_option: None,
            response_value_render_option: None,
            value_input_option: Some("USER_ENTERED".to_string()),
        };

        tracing::trace!("update_request: {:?}", update_request);

        let response = self
            .hub
            .spreadsheets()
            .values_batch_update(update_request, &sheet_key)
            .doit()
            .await;

        match response {
            Ok(my_result) => Ok(my_result.1.total_updated_cells.unwrap()),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}
