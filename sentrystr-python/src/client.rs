use pyo3::prelude::*;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};

use sentrystr::NostrSentryClient;
use crate::{PyConfig, PyEvent};

#[pyclass(name = "NostrSentryClient")]
pub struct PyNostrSentryClient {
    inner: Arc<Mutex<NostrSentryClient>>,
    runtime: Arc<Runtime>,
}

#[pymethods]
impl PyNostrSentryClient {
    #[new]
    pub fn new(config: &PyConfig) -> PyResult<Self> {
        let runtime = Arc::new(
            Runtime::new()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
        );

        let client = runtime.block_on(async {
            NostrSentryClient::new(config.inner().clone()).await
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(client)),
            runtime,
        })
    }

    pub fn capture_event(&self, event: &PyEvent) -> PyResult<()> {
        let event = event.inner().clone();
        self.runtime.block_on(async {
            let client = self.inner.lock().unwrap();
            client
                .capture_event(event)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
                .map(|_| ()) // Ignore the EventId return value
        })
    }

    pub fn capture_message(&self, message: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let client = self.inner.lock().unwrap();
            client
                .capture_message(&message)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
                .map(|_| ()) // Ignore the EventId return value
        })
    }

    pub fn capture_error(&self, error: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let client = self.inner.lock().unwrap();
            client
                .capture_error(&error)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
                .map(|_| ()) // Ignore the EventId return value
        })
    }

    pub fn capture_exception(&self, exception_type: String, message: Option<String>) -> PyResult<()> {
        self.runtime.block_on(async {
            // Create a simple exception event instead
            let event = sentrystr::Event::new()
                .with_message(message.unwrap_or_else(|| exception_type.clone()))
                .with_level(sentrystr::Level::Error);

            let client = self.inner.lock().unwrap();
            client
                .capture_event(event)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
                .map(|_| ()) // Ignore the EventId return value
        })
    }

    pub fn send_direct_message(&self, content: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let client = self.inner.lock().unwrap();
            client
                .send_direct_message(&content)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
        })
    }

    pub fn setup_direct_messaging(&self, recipient_npub: String) -> PyResult<()> {
        use nostr::prelude::*;
        use nostr_sdk::prelude::*;
        use sentrystr::{DirectMessageBuilder, Level};
        use std::str::FromStr;

        self.runtime.block_on(async {
            // Parse the recipient public key
            let recipient_pubkey = PublicKey::from_str(&recipient_npub)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid pubkey: {}", e)))?;

            // Generate keys for the DM client
            let keys = Keys::generate();
            let nostr_client = Client::new(keys.clone());

            // Add the same relays as the main client (simplified approach)
            nostr_client.add_relay("wss://relay.damus.io").await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            nostr_client.add_relay("wss://nos.lol").await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            nostr_client.add_relay("wss://nostr.chaima.info").await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            nostr_client.connect().await;

            // Build the DirectMessageSender
            let dm_sender = DirectMessageBuilder::new()
                .with_client(nostr_client)
                .with_keys(keys)
                .with_recipient(recipient_pubkey)
                .with_min_level(Level::Warning) // Only send DMs for warnings and above
                .with_nip17(true) // Use NIP-17 for better privacy
                .build()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            // Set up direct messaging on the client
            let mut client = self.inner.lock().unwrap();
            client.set_direct_messaging(dm_sender);

            Ok(())
        })
    }

    fn __repr__(&self) -> String {
        "NostrSentryClient()".to_string()
    }
}