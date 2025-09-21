use crate::{convert_tracing_level, create_sentrystr_event, extract_event_metadata, FieldVisitor};
use sentrystr::{DirectMessageSender, MessageEvent, NostrSentryClient};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

pub struct SentryStrLayer {
    client: Arc<RwLock<NostrSentryClient>>,
    dm_sender: Option<Arc<RwLock<DirectMessageSender>>>,
    min_level: Option<tracing::Level>,
    include_fields: bool,
    include_metadata: bool,
}

impl SentryStrLayer {
    pub fn new(client: NostrSentryClient) -> Self {
        Self {
            client: Arc::new(RwLock::new(client)),
            dm_sender: None,
            min_level: None,
            include_fields: true,
            include_metadata: true,
        }
    }

    pub fn with_direct_messaging(mut self, dm_sender: DirectMessageSender) -> Self {
        self.dm_sender = Some(Arc::new(RwLock::new(dm_sender)));
        self
    }

    pub fn with_min_level(mut self, level: tracing::Level) -> Self {
        self.min_level = Some(level);
        self
    }

    pub fn with_fields(mut self, include: bool) -> Self {
        self.include_fields = include;
        self
    }

    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    fn should_process_event(&self, event_level: &tracing::Level) -> bool {
        if let Some(min_level) = &self.min_level {
            event_level <= min_level
        } else {
            true
        }
    }
}

impl<S> Layer<S> for SentryStrLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);

        let message = visitor.extract_message();
        let level = convert_tracing_level(event.metadata().level());

        if !self.should_process_event(event.metadata().level()) {
            return;
        }

        let fields = if self.include_fields {
            visitor.fields
        } else {
            std::collections::BTreeMap::new()
        };

        let metadata_fields = if self.include_metadata {
            extract_event_metadata(event.metadata())
        } else {
            std::collections::BTreeMap::new()
        };

        let sentrystr_event = create_sentrystr_event(message, level, fields, metadata_fields);

        let client = Arc::clone(&self.client);
        let dm_sender = self.dm_sender.as_ref().map(Arc::clone);

        tokio::spawn(async move {
            let client = client.read().await;
            if let Err(e) = client.capture_event(sentrystr_event.clone()).await {
                eprintln!("Failed to send event to SentryStr: {}", e);
                return;
            }

            if let Some(dm_sender) = dm_sender {
                let dm_sender = dm_sender.read().await;
                let message_event = MessageEvent {
                    event: sentrystr_event,
                    author: nostr::Keys::generate().public_key(),
                    nostr_event_id: nostr::EventId::all_zeros(),
                    received_at: chrono::Utc::now(),
                };

                if let Err(e) = dm_sender.send_message_for_event(&message_event).await {
                    eprintln!("Failed to send direct message: {}", e);
                }
            }
        });
    }
}

impl Clone for SentryStrLayer {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            dm_sender: self.dm_sender.as_ref().map(Arc::clone),
            min_level: self.min_level,
            include_fields: self.include_fields,
            include_metadata: self.include_metadata,
        }
    }
}
