use pyo3::prelude::*;
use sentrystr::{Config, EncryptionVersion};

#[pyclass(name = "Config")]
#[derive(Debug, Clone)]
pub struct PyConfig {
    inner: Config,
}

#[pymethods]
impl PyConfig {
    #[new]
    pub fn new(private_key: String, relays: Vec<String>) -> PyResult<Self> {
        let config = Config::new(private_key, relays);
        Ok(Self { inner: config })
    }

    #[getter]
    pub fn relays(&self) -> Vec<String> {
        self.inner.relays.clone()
    }

    #[setter]
    pub fn set_relays(&mut self, relays: Vec<String>) {
        self.inner.relays = relays;
    }

    #[getter]
    pub fn encrypt_events(&self) -> bool {
        self.inner.encrypt_events
    }

    #[setter]
    pub fn set_encrypt_events(&mut self, encrypt_events: bool) {
        self.inner.encrypt_events = encrypt_events;
    }

    #[getter]
    pub fn recipient_pubkey(&self) -> Option<String> {
        self.inner.recipient_pubkey.clone()
    }

    #[setter]
    pub fn set_recipient_pubkey(&mut self, recipient_pubkey: Option<String>) {
        self.inner.recipient_pubkey = recipient_pubkey;
    }

    #[getter]
    pub fn event_kind(&self) -> u16 {
        self.inner.event_kind
    }

    #[setter]
    pub fn set_event_kind(&mut self, event_kind: u16) {
        self.inner.event_kind = event_kind;
    }

    pub fn with_encryption(&mut self, recipient_pubkey: String) {
        self.inner = self.inner.clone().with_encryption(recipient_pubkey);
    }

    pub fn with_nip44_encryption(&mut self, recipient_pubkey: String) {
        self.inner = self.inner.clone().with_nip44_encryption(recipient_pubkey);
    }

    pub fn with_encryption_version(&mut self, version: String) -> PyResult<()> {
        let encryption_version = match version.as_str() {
            "none" => EncryptionVersion::None,
            "nip44v2" => EncryptionVersion::Nip44V2,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid encryption version. Must be 'none' or 'nip44v2'"
            )),
        };
        self.inner.encryption_version = encryption_version;
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("Config(relays={:?}, encrypt_events={})", self.inner.relays, self.inner.encrypt_events)
    }
}

impl PyConfig {
    pub fn inner(&self) -> &Config {
        &self.inner
    }

    pub fn into_inner(self) -> Config {
        self.inner
    }
}