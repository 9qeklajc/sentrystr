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
    pub fn new(py: Python<'_>, config: &PyConfig) -> PyResult<Self> {
        let runtime = Arc::new(
            Runtime::new()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
        );

        // Clone the plain config out of the pyclass so the closure captures only
        // `Ungil` data, then release the GIL: connecting to relays is a blocking
        // network round-trip and must not freeze other Python threads.
        let config = config.inner().clone();
        let rt = runtime.clone();
        let client = py
            .detach(move || rt.block_on(NostrSentryClient::new(config)))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(client)),
            runtime,
        })
    }

    pub fn capture_event(&self, py: Python<'_>, event: &PyEvent) -> PyResult<()> {
        // Clone out of the pyclasses so the GIL-released closure captures only
        // `Send + Ungil` data. `PyErr` is `!Ungil`, so the blocking work returns
        // the native error and we build the `PyErr` after re-acquiring the GIL.
        let event = event.inner().clone();
        let inner = self.inner.clone();
        let runtime = self.runtime.clone();
        py.detach(move || {
            runtime.block_on(async move {
                let client = inner.lock().unwrap();
                client.capture_event(event).await.map(|_| ())
            })
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    pub fn capture_message(&self, py: Python<'_>, message: String) -> PyResult<()> {
        let inner = self.inner.clone();
        let runtime = self.runtime.clone();
        py.detach(move || {
            runtime.block_on(async move {
                let client = inner.lock().unwrap();
                client.capture_message(&message).await.map(|_| ())
            })
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    pub fn capture_error(&self, py: Python<'_>, error: String) -> PyResult<()> {
        let inner = self.inner.clone();
        let runtime = self.runtime.clone();
        py.detach(move || {
            runtime.block_on(async move {
                let client = inner.lock().unwrap();
                client.capture_error(&error).await.map(|_| ())
            })
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    pub fn capture_exception(&self, py: Python<'_>, exception_type: String, message: Option<String>) -> PyResult<()> {
        let inner = self.inner.clone();
        let runtime = self.runtime.clone();
        py.detach(move || {
            runtime.block_on(async move {
                // Create a simple exception event instead
                let event = sentrystr::Event::new()
                    .with_message(message.unwrap_or_else(|| exception_type.clone()))
                    .with_level(sentrystr::Level::Error);

                let client = inner.lock().unwrap();
                client.capture_event(event).await.map(|_| ())
            })
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    pub fn send_direct_message(&self, py: Python<'_>, content: String) -> PyResult<()> {
        let inner = self.inner.clone();
        let runtime = self.runtime.clone();
        py.detach(move || {
            runtime.block_on(async move {
                let client = inner.lock().unwrap();
                client.send_direct_message(&content).await
            })
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
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