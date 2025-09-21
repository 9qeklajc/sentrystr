use crate::{EventFilter, Result};
use chrono::{DateTime, Utc};
use nostr::prelude::*;
use nostr_sdk::prelude::*;
use sentrystr::{DirectMessageBuilder, DirectMessageSender, Event, Level, MessageEvent};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct CollectedEvent {
    pub event: Event,
    pub author: PublicKey,
    pub nostr_event_id: EventId,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PrivateMessageConfig {
    pub recipient_pubkey: PublicKey,
    pub min_level: Option<Level>,
    pub use_nip17: bool,
}

/// Collects and monitors SentryStr events from Nostr relays.
///
/// # Examples
///
/// ```rust
/// use sentrystr_collector::{EventCollector, EventFilter};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let relays = vec!["wss://relay.damus.io".to_string()];
/// let collector = EventCollector::new(relays).await?;
///
/// let events = collector.collect_events(EventFilter::new().with_limit(10)).await?;
/// println!("Collected {} events", events.len());
/// # Ok(())
/// # }
/// ```
pub struct EventCollector {
    client: Client,
    keys: Keys,
    event_kind: u16,
    dm_sender: Option<DirectMessageSender>,
}

impl EventCollector {
    /// Creates a new EventCollector connected to the specified relays.
    pub async fn new(relays: Vec<String>) -> Result<Self> {
        let keys = Keys::generate();
        let client = Client::new(keys.clone());

        for relay in &relays {
            client.add_relay(relay).await?;
        }

        client.connect().await;

        Ok(Self {
            client,
            keys,
            event_kind: 9898,
            dm_sender: None,
        })
    }

    pub fn with_private_messaging(mut self, config: PrivateMessageConfig) -> Result<Self> {
        let dm_sender = DirectMessageBuilder::new()
            .with_client(self.client.clone())
            .with_keys(self.keys.clone())
            .with_recipient(config.recipient_pubkey)
            .with_min_level(config.min_level.unwrap_or(Level::Debug))
            .with_nip17(config.use_nip17)
            .build()
            .map_err(|e| {
                crate::CollectorError::Collection(format!("Failed to create DM sender: {}", e))
            })?;

        self.dm_sender = Some(dm_sender);
        Ok(self)
    }

    pub async fn collect_events(&self, filter: EventFilter) -> Result<Vec<CollectedEvent>> {
        let mut nostr_filter = Filter::new().kind(Kind::Custom(self.event_kind));

        if let Some(ref authors) = filter.authors {
            let author_keys: Vec<PublicKey> = authors.iter().cloned().collect();
            nostr_filter = nostr_filter.authors(author_keys);
        }

        if let Some(since) = filter.since {
            nostr_filter = nostr_filter.since(Timestamp::from_secs(since.timestamp() as u64));
        }

        if let Some(until) = filter.until {
            nostr_filter = nostr_filter.until(Timestamp::from_secs(until.timestamp() as u64));
        }

        if let Some(limit) = filter.limit {
            nostr_filter = nostr_filter.limit(limit);
        }

        let events = self
            .client
            .fetch_events(nostr_filter, std::time::Duration::from_secs(10))
            .await?;

        let mut collected_events = Vec::new();

        for event in events {
            if let Ok(parsed_event) = serde_json::from_str::<Event>(&event.content) {
                if filter.matches_nostr_event(&parsed_event, &event.pubkey, &event) {
                    let collected_event = CollectedEvent {
                        event: parsed_event.clone(),
                        author: event.pubkey,
                        nostr_event_id: event.id,
                        received_at: Utc::now(),
                    };

                    // Send private message if configured
                    if let Some(ref dm_sender) = self.dm_sender {
                        let message_event = MessageEvent {
                            event: parsed_event.clone(),
                            author: event.pubkey,
                            nostr_event_id: event.id,
                            received_at: Utc::now(),
                        };

                        if let Err(e) = dm_sender.send_message_for_event(&message_event).await {
                            eprintln!("Failed to send direct message: {}", e);
                        }
                    }

                    collected_events.push(collected_event);
                }
            }
        }

        Ok(collected_events)
    }

    pub async fn subscribe_to_events(
        &self,
        filter: EventFilter,
    ) -> Result<mpsc::Receiver<CollectedEvent>> {
        let (tx, rx) = mpsc::channel(1000);

        let mut nostr_filter = Filter::new().kind(Kind::Custom(self.event_kind));

        if let Some(ref authors) = filter.authors {
            let author_keys: Vec<PublicKey> = authors.iter().cloned().collect();
            nostr_filter = nostr_filter.authors(author_keys);
        }

        if let Some(since) = filter.since {
            nostr_filter = nostr_filter.since(Timestamp::from_secs(since.timestamp() as u64));
        }

        let subscription_id = self.client.subscribe(nostr_filter, None).await?;

        let client_clone = self.client.clone();
        let _keys_clone = self.keys.clone();
        let filter_clone = filter.clone();
        let dm_sender_clone = self.dm_sender.clone();

        tokio::spawn(async move {
            let mut notifications = client_clone.notifications();

            while let Ok(notification) = notifications.recv().await {
                if let RelayPoolNotification::Event {
                    subscription_id: sub_id,
                    event,
                    ..
                } = notification
                {
                    if sub_id == subscription_id.val {
                        if let Ok(parsed_event) = serde_json::from_str::<Event>(&event.content) {
                            if filter_clone.matches_nostr_event(
                                &parsed_event,
                                &event.pubkey,
                                &event,
                            ) {
                                let collected_event = CollectedEvent {
                                    event: parsed_event.clone(),
                                    author: event.pubkey,
                                    nostr_event_id: event.id,
                                    received_at: Utc::now(),
                                };

                                if let Some(ref dm_sender) = dm_sender_clone {
                                    let message_event = MessageEvent {
                                        event: parsed_event.clone(),
                                        author: event.pubkey,
                                        nostr_event_id: event.id,
                                        received_at: Utc::now(),
                                    };

                                    if let Err(e) =
                                        dm_sender.send_message_for_event(&message_event).await
                                    {
                                        eprintln!("Failed to send direct message: {}", e);
                                    }
                                }

                                if tx.send(collected_event).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    pub async fn get_events_by_author(
        &self,
        author: PublicKey,
        limit: Option<usize>,
    ) -> Result<Vec<CollectedEvent>> {
        let filter = EventFilter::new()
            .with_author(author)
            .with_limit(limit.unwrap_or(100));

        self.collect_events(filter).await
    }

    pub async fn disconnect(&self) -> Result<()> {
        self.client.disconnect().await;
        Ok(())
    }
}
