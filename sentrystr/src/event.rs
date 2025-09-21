use chrono::{DateTime, Utc};
use nostr::prelude::Tag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub platform: String,
    pub level: Level,
    pub logger: Option<String>,
    pub transaction: Option<String>,
    pub server_name: Option<String>,
    pub release: Option<String>,
    pub environment: Option<String>,
    pub message: Option<String>,
    pub exception: Option<Vec<Exception>>,
    pub stacktrace: Option<Stacktrace>,
    pub user: Option<User>,
    pub request: Option<Request>,
    pub tags: HashMap<String, String>,
    pub extra: HashMap<String, serde_json::Value>,
    pub fingerprint: Option<Vec<String>>,
    pub modules: Option<HashMap<String, String>>,
    pub nostr_tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    #[serde(rename = "type")]
    pub exception_type: String,
    pub value: String,
    pub module: Option<String>,
    pub stacktrace: Option<Stacktrace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stacktrace {
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub filename: String,
    pub function: Option<String>,
    pub module: Option<String>,
    pub lineno: Option<u32>,
    pub colno: Option<u32>,
    pub abs_path: Option<String>,
    pub context_line: Option<String>,
    pub pre_context: Option<Vec<String>>,
    pub post_context: Option<Vec<String>>,
    pub in_app: Option<bool>,
    pub vars: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub query_string: Option<String>,
    pub cookies: Option<String>,
    pub data: Option<serde_json::Value>,
    pub env: Option<HashMap<String, String>>,
}

impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}

impl Event {
    pub fn new() -> Self {
        Self {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            platform: "rust".to_string(),
            level: Level::Info,
            logger: None,
            transaction: None,
            server_name: None,
            release: None,
            environment: None,
            message: None,
            exception: None,
            stacktrace: None,
            user: None,
            request: None,
            tags: HashMap::new(),
            extra: HashMap::new(),
            fingerprint: None,
            modules: None,
            nostr_tags: Vec::new(),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn with_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }

    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_exception(mut self, exception: Exception) -> Self {
        match self.exception {
            Some(ref mut exceptions) => exceptions.push(exception),
            None => self.exception = Some(vec![exception]),
        }
        self
    }

    pub fn with_nostr_tag(mut self, tag: Tag) -> Self {
        self.nostr_tags.push(tag);
        self
    }

    pub fn with_nostr_tags(mut self, tags: Vec<Tag>) -> Self {
        self.nostr_tags.extend(tags);
        self
    }

    pub fn with_service_tag(mut self, service: impl Into<String>) -> Self {
        self.nostr_tags
            .push(Tag::parse(vec!["service", &service.into()]).unwrap());
        self
    }

    pub fn with_environment_tag(mut self, environment: impl Into<String>) -> Self {
        self.nostr_tags
            .push(Tag::parse(vec!["env", &environment.into()]).unwrap());
        self
    }

    pub fn with_severity_tag(mut self, level: &Level) -> Self {
        let severity = match level {
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warning => "warning",
            Level::Error => "error",
            Level::Fatal => "fatal",
        };
        self.nostr_tags
            .push(Tag::parse(vec!["severity", severity]).unwrap());
        self
    }

    pub fn with_component_tag(mut self, component: impl Into<String>) -> Self {
        self.nostr_tags
            .push(Tag::parse(vec!["component", &component.into()]).unwrap());
        self
    }
}
