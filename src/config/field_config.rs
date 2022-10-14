use super::field_config_entry::FieldConfigEntry;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FieldConfig {
    pub sheet_field: Vec<FieldConfigEntry>,
}
