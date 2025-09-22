use pyo3::prelude::*;

mod client;
mod config;
mod event;
mod error;

pub use client::*;
pub use config::*;
pub use event::*;
pub use error::*;

#[pymodule]
fn _sentrystr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyNostrSentryClient>()?;
    m.add_class::<PyConfig>()?;
    m.add_class::<PyEvent>()?;
    m.add_class::<PyLevel>()?;
    m.add_class::<PyException>()?;
    m.add_class::<PyStacktrace>()?;
    m.add_class::<PyFrame>()?;
    m.add_class::<PyUser>()?;
    m.add_class::<PyRequest>()?;
    m.add_class::<PySentryStrError>()?;

    Ok(())
}