use serde_json::Value;
use std::collections::BTreeMap;
use tracing::field::{Field, Visit};

pub struct FieldVisitor {
    pub fields: BTreeMap<String, Value>,
    pub message: Option<String>,
}

impl FieldVisitor {
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
            message: None,
        }
    }

    pub fn extract_message(&self) -> String {
        self.message
            .clone()
            .or_else(|| {
                self.fields
                    .get("message")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "No message".to_string())
    }
}

impl Visit for FieldVisitor {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.fields.insert(
            field.name().to_string(),
            Value::Number(
                serde_json::Number::from_f64(value).unwrap_or_else(|| serde_json::Number::from(0)),
            ),
        );
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields.insert(
            field.name().to_string(),
            Value::Number(serde_json::Number::from(value)),
        );
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields.insert(
            field.name().to_string(),
            Value::Number(serde_json::Number::from(value)),
        );
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .insert(field.name().to_string(), Value::Bool(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        let field_name = field.name();
        let value_string = value.to_string();

        if field_name == "message" {
            self.message = Some(value_string.clone());
        }

        self.fields
            .insert(field_name.to_string(), Value::String(value_string));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(
            field.name().to_string(),
            Value::String(format!("{:?}", value)),
        );
    }
}

impl Default for FieldVisitor {
    fn default() -> Self {
        Self::new()
    }
}
