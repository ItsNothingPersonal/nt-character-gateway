use super::field_config_entry::FieldConfigEntry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FieldConfig {
    pub sheet_field: Vec<FieldConfigEntry>,
}
