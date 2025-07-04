use serde_json::Value;

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_required_fields(config: &Value) -> Result<(), String> {
        let required_fields = vec!["profession", "supabaseUrl", "supabaseApiKey"];

        for field in required_fields {
            if config.get(field).is_none() {
                return Err(format!("Missing required field: {field}"));
            }
        }

        Ok(())
    }

    pub fn validate_import_config(config: &Value) -> Result<(), String> {
        Self::validate_required_fields(config)?;

        // Additional validation for imported configs
        if let Some(url) = config.get("supabaseUrl").and_then(|v| v.as_str()) {
            if !url.starts_with("https://") {
                return Err("Supabase URL must start with https://".to_string());
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn validate_template_config(config: &Value) -> Result<(), String> {
        // Basic validation for template configs
        if config.get("profession").is_none() {
            return Err("Template must have a profession".to_string());
        }

        Ok(())
    }
}
