use config::Config;

use super::{
    field_config::FieldConfig, field_config_entry::FieldConfigEntry, field_name::FieldName,
};

#[derive(Clone)]
pub struct ConfigClient {
    pub settings: FieldConfig,
}

impl ConfigClient {
    pub fn new() -> ConfigClient {
        let settings: FieldConfig = Config::builder()
            .add_source(config::File::with_name("FieldConfig.toml"))
            .build()
            .unwrap()
            .try_deserialize::<FieldConfig>()
            .unwrap();

        ConfigClient { settings }
    }

    pub fn get_field_config(&self, field_name: FieldName) -> FieldConfigEntry {
        let sheet_fields: Vec<FieldConfigEntry> = self.settings.sheet_field.clone();
        sheet_fields
            .iter()
            .find(|&x| -> bool { x.name == field_name })
            .unwrap()
            .clone()
    }

    pub fn get_field_config_sorted(&self) -> Vec<FieldConfigEntry> {
        let mut sheet_fields = self.settings.sheet_field.clone();
        sheet_fields.sort_by(|a, b| a.position.cmp(&b.position));
        sheet_fields
    }
}
