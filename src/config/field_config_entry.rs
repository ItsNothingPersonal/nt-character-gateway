use super::field_name::FieldName;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FieldConfigEntry {
    pub name: FieldName,
    pub position: u8,
    pub range: String,
    pub range_length: Option<u8>,
    pub exclude_on_read: Option<bool>,
}
