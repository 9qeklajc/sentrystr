use chrono::{DateTime, Utc};
use nostr::PublicKey;
use sentrystr::{Event, Level};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct EventFilter {
    pub authors: Option<HashSet<PublicKey>>,
    pub levels: Option<HashSet<Level>>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub tags: Option<Vec<(String, String)>>,
    pub nostr_tags: Option<Vec<(String, String)>>,
    pub limit: Option<usize>,
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            authors: None,
            levels: None,
            since: None,
            until: None,
            tags: None,
            nostr_tags: None,
            limit: None,
        }
    }

    pub fn with_author(mut self, author: PublicKey) -> Self {
        match self.authors {
            Some(ref mut authors) => {
                authors.insert(author);
            }
            None => {
                let mut authors = HashSet::new();
                authors.insert(author);
                self.authors = Some(authors);
            }
        }
        self
    }

    pub fn with_level(mut self, level: Level) -> Self {
        match self.levels {
            Some(ref mut levels) => {
                levels.insert(level);
            }
            None => {
                let mut levels = HashSet::new();
                levels.insert(level);
                self.levels = Some(levels);
            }
        }
        self
    }

    pub fn with_since(mut self, since: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self
    }

    pub fn with_until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until);
        self
    }

    pub fn with_tag(mut self, key: String, value: String) -> Self {
        match self.tags {
            Some(ref mut tags) => tags.push((key, value)),
            None => self.tags = Some(vec![(key, value)]),
        }
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_nostr_tag(mut self, key: String, value: String) -> Self {
        match self.nostr_tags {
            Some(ref mut tags) => tags.push((key, value)),
            None => self.nostr_tags = Some(vec![(key, value)]),
        }
        self
    }

    pub fn with_service_filter(self, service: String) -> Self {
        self.with_nostr_tag("service".to_string(), service)
    }

    pub fn with_environment_filter(self, environment: String) -> Self {
        self.with_nostr_tag("env".to_string(), environment)
    }

    pub fn with_component_filter(self, component: String) -> Self {
        self.with_nostr_tag("component".to_string(), component)
    }

    pub fn with_severity_filter(self, severity: String) -> Self {
        self.with_nostr_tag("severity".to_string(), severity)
    }

    pub fn matches(&self, event: &Event, author: &PublicKey) -> bool {
        if let Some(ref authors) = self.authors {
            if !authors.contains(author) {
                return false;
            }
        }

        if let Some(ref levels) = self.levels {
            if !levels.contains(&event.level) {
                return false;
            }
        }

        if let Some(since) = self.since {
            if event.timestamp < since {
                return false;
            }
        }

        if let Some(until) = self.until {
            if event.timestamp > until {
                return false;
            }
        }

        if let Some(ref filter_tags) = self.tags {
            for (key, value) in filter_tags {
                if let Some(event_value) = event.tags.get(key) {
                    if event_value != value {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    pub fn matches_nostr_event(
        &self,
        parsed_event: &Event,
        author: &PublicKey,
        nostr_event: &nostr::Event,
    ) -> bool {
        if !self.matches(parsed_event, author) {
            return false;
        }

        if let Some(ref filter_nostr_tags) = self.nostr_tags {
            for (key, value) in filter_nostr_tags {
                let mut found = false;
                for tag in nostr_event.tags.iter() {
                    let tag_vec = tag.clone().to_vec();
                    if let Some(tag_key) = tag_vec.first() {
                        if tag_key == key {
                            if let Some(tag_value) = tag_vec.get(1) {
                                if tag_value == value {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !found {
                    return false;
                }
            }
        }

        true
    }
}
