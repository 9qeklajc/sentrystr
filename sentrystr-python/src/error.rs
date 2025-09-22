use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use sentrystr::SentryStrError;

#[pyclass(name = "SentryStrError")]
#[derive(Debug, Clone)]
pub struct PySentryStrError {
    message: String,
}

#[pymethods]
impl PySentryStrError {
    #[new]
    pub fn new(message: String) -> Self {
        Self { message }
    }

    fn __str__(&self) -> &str {
        &self.message
    }

    fn __repr__(&self) -> String {
        format!("SentryStrError('{}')", self.message)
    }
}

// Convert SentryStrError to PyErr using a function instead of From trait to avoid orphan rules
pub fn sentrystr_error_to_pyerr(err: SentryStrError) -> PyErr {
    PyException::new_err(err.to_string())
}

impl From<SentryStrError> for PySentryStrError {
    fn from(err: SentryStrError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}