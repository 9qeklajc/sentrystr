use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use sentrystr::{Event, Level, Exception, Stacktrace, Frame, User, Request};
use std::collections::HashMap;

#[pyclass(name = "Level")]
#[derive(Debug, Clone)]
pub enum PyLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

#[pymethods]
impl PyLevel {
    #[new]
    pub fn new(level: &str) -> PyResult<Self> {
        match level.to_lowercase().as_str() {
            "debug" => Ok(PyLevel::Debug),
            "info" => Ok(PyLevel::Info),
            "warning" | "warn" => Ok(PyLevel::Warning),
            "error" => Ok(PyLevel::Error),
            "fatal" => Ok(PyLevel::Fatal),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid level. Must be one of: debug, info, warning, error, fatal"
            )),
        }
    }

    fn __str__(&self) -> &'static str {
        match self {
            PyLevel::Debug => "debug",
            PyLevel::Info => "info",
            PyLevel::Warning => "warning",
            PyLevel::Error => "error",
            PyLevel::Fatal => "fatal",
        }
    }

    fn __repr__(&self) -> String {
        format!("Level.{}", self.__str__().to_uppercase())
    }
}

impl From<PyLevel> for Level {
    fn from(py_level: PyLevel) -> Self {
        match py_level {
            PyLevel::Debug => Level::Debug,
            PyLevel::Info => Level::Info,
            PyLevel::Warning => Level::Warning,
            PyLevel::Error => Level::Error,
            PyLevel::Fatal => Level::Fatal,
        }
    }
}

impl From<Level> for PyLevel {
    fn from(level: Level) -> Self {
        match level {
            Level::Debug => PyLevel::Debug,
            Level::Info => PyLevel::Info,
            Level::Warning => PyLevel::Warning,
            Level::Error => PyLevel::Error,
            Level::Fatal => PyLevel::Fatal,
        }
    }
}

#[pyclass(name = "Frame")]
#[derive(Debug, Clone)]
pub struct PyFrame {
    inner: Frame,
}

#[pymethods]
impl PyFrame {
    #[new]
    pub fn new(filename: String) -> Self {
        Self {
            inner: Frame {
                filename,
                function: None,
                module: None,
                lineno: None,
                colno: None,
                abs_path: None,
                context_line: None,
                pre_context: None,
                post_context: None,
                in_app: None,
                vars: None,
            },
        }
    }

    #[getter]
    pub fn filename(&self) -> String {
        self.inner.filename.clone()
    }

    #[setter]
    pub fn set_filename(&mut self, filename: String) {
        self.inner.filename = filename;
    }

    #[getter]
    pub fn function(&self) -> Option<String> {
        self.inner.function.clone()
    }

    #[setter]
    pub fn set_function(&mut self, function: Option<String>) {
        self.inner.function = function;
    }

    #[getter]
    pub fn lineno(&self) -> Option<u32> {
        self.inner.lineno
    }

    #[setter]
    pub fn set_lineno(&mut self, lineno: Option<u32>) {
        self.inner.lineno = lineno;
    }

    #[getter]
    pub fn colno(&self) -> Option<u32> {
        self.inner.colno
    }

    #[setter]
    pub fn set_colno(&mut self, colno: Option<u32>) {
        self.inner.colno = colno;
    }

    pub fn with_function(&mut self, function: String) {
        self.inner.function = Some(function);
    }

    pub fn with_lineno(&mut self, lineno: u32) {
        self.inner.lineno = Some(lineno);
    }

    pub fn with_colno(&mut self, colno: u32) {
        self.inner.colno = Some(colno);
    }
}

impl From<PyFrame> for Frame {
    fn from(py_frame: PyFrame) -> Self {
        py_frame.inner
    }
}

#[pyclass(name = "Stacktrace")]
#[derive(Debug, Clone)]
pub struct PyStacktrace {
    inner: Stacktrace,
}

#[pymethods]
impl PyStacktrace {
    #[new]
    pub fn new(frames: Vec<PyRef<'_, PyFrame>>) -> Self {
        let frames: Vec<Frame> = frames.into_iter().map(|f| f.inner.clone()).collect();
        Self {
            inner: Stacktrace { frames },
        }
    }

    #[getter]
    pub fn frames(&self) -> Vec<PyFrame> {
        self.inner.frames.iter().map(|f| PyFrame { inner: f.clone() }).collect()
    }
}

impl From<PyStacktrace> for Stacktrace {
    fn from(py_stacktrace: PyStacktrace) -> Self {
        py_stacktrace.inner
    }
}

#[pyclass(name = "Exception")]
#[derive(Debug, Clone)]
pub struct PyException {
    inner: Exception,
}

#[pymethods]
impl PyException {
    #[new]
    pub fn new(exception_type: String, value: String) -> Self {
        Self {
            inner: Exception {
                exception_type,
                value,
                module: None,
                stacktrace: None,
            },
        }
    }

    #[getter]
    pub fn exception_type(&self) -> String {
        self.inner.exception_type.clone()
    }

    #[getter]
    pub fn value(&self) -> String {
        self.inner.value.clone()
    }

    #[getter]
    pub fn module(&self) -> Option<String> {
        self.inner.module.clone()
    }

    #[setter]
    pub fn set_module(&mut self, module: Option<String>) {
        self.inner.module = module;
    }

    pub fn with_module(&mut self, module: String) {
        self.inner.module = Some(module);
    }

    pub fn with_stacktrace(&mut self, stacktrace: &PyStacktrace) {
        self.inner.stacktrace = Some(stacktrace.inner.clone());
    }
}

impl From<PyException> for Exception {
    fn from(py_exception: PyException) -> Self {
        py_exception.inner
    }
}

#[pyclass(name = "User")]
#[derive(Debug, Clone)]
pub struct PyUser {
    inner: User,
}

#[pymethods]
impl PyUser {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: User {
                id: None,
                username: None,
                email: None,
                ip_address: None,
            },
        }
    }

    #[getter]
    pub fn id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    #[setter]
    pub fn set_id(&mut self, id: Option<String>) {
        self.inner.id = id;
    }

    #[getter]
    pub fn email(&self) -> Option<String> {
        self.inner.email.clone()
    }

    #[setter]
    pub fn set_email(&mut self, email: Option<String>) {
        self.inner.email = email;
    }

    #[getter]
    pub fn username(&self) -> Option<String> {
        self.inner.username.clone()
    }

    #[setter]
    pub fn set_username(&mut self, username: Option<String>) {
        self.inner.username = username;
    }

    pub fn with_id(&mut self, id: String) {
        self.inner.id = Some(id);
    }

    pub fn with_email(&mut self, email: String) {
        self.inner.email = Some(email);
    }

    pub fn with_username(&mut self, username: String) {
        self.inner.username = Some(username);
    }
}

impl From<PyUser> for User {
    fn from(py_user: PyUser) -> Self {
        py_user.inner
    }
}

#[pyclass(name = "Request")]
#[derive(Debug, Clone)]
pub struct PyRequest {
    inner: Request,
}

#[pymethods]
impl PyRequest {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Request {
                url: None,
                method: None,
                headers: None,
                query_string: None,
                cookies: None,
                data: None,
                env: None,
            },
        }
    }

    #[getter]
    pub fn url(&self) -> Option<String> {
        self.inner.url.clone()
    }

    #[setter]
    pub fn set_url(&mut self, url: Option<String>) {
        self.inner.url = url;
    }

    #[getter]
    pub fn method(&self) -> Option<String> {
        self.inner.method.clone()
    }

    #[setter]
    pub fn set_method(&mut self, method: Option<String>) {
        self.inner.method = method;
    }

    #[getter]
    pub fn query_string(&self) -> Option<String> {
        self.inner.query_string.clone()
    }

    #[setter]
    pub fn set_query_string(&mut self, query_string: Option<String>) {
        self.inner.query_string = query_string;
    }

    pub fn with_url(&mut self, url: String) {
        self.inner.url = Some(url);
    }

    pub fn with_method(&mut self, method: String) {
        self.inner.method = Some(method);
    }

    pub fn with_query_string(&mut self, query_string: String) {
        self.inner.query_string = Some(query_string);
    }
}

impl From<PyRequest> for Request {
    fn from(py_request: PyRequest) -> Self {
        py_request.inner
    }
}

#[pyclass(name = "Event")]
#[derive(Debug, Clone)]
pub struct PyEvent {
    inner: Event,
}

#[pymethods]
impl PyEvent {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Event::new(),
        }
    }

    #[getter]
    pub fn event_id(&self) -> String {
        self.inner.event_id.clone()
    }

    #[getter]
    pub fn level(&self) -> PyLevel {
        self.inner.level.clone().into()
    }

    #[setter]
    pub fn set_level(&mut self, level: PyLevel) {
        self.inner.level = level.into();
    }

    #[getter]
    pub fn message(&self) -> Option<String> {
        self.inner.message.clone()
    }

    #[setter]
    pub fn set_message(&mut self, message: Option<String>) {
        self.inner.message = message;
    }

    #[getter]
    pub fn logger(&self) -> Option<String> {
        self.inner.logger.clone()
    }

    #[setter]
    pub fn set_logger(&mut self, logger: Option<String>) {
        self.inner.logger = logger;
    }

    #[getter]
    pub fn platform(&self) -> String {
        self.inner.platform.clone()
    }

    #[setter]
    pub fn set_platform(&mut self, platform: String) {
        self.inner.platform = platform;
    }

    pub fn with_message(&mut self, message: String) {
        self.inner = self.inner.clone().with_message(message);
    }

    pub fn with_level(&mut self, level: PyLevel) {
        self.inner = self.inner.clone().with_level(level.into());
    }

    pub fn with_user(&mut self, user: &PyUser) {
        self.inner = self.inner.clone().with_user(user.inner.clone());
    }

    pub fn with_exception(&mut self, exception: &PyException) {
        self.inner = self.inner.clone().with_exception(exception.inner.clone());
    }

    pub fn with_tag(&mut self, key: String, value: String) {
        self.inner = self.inner.clone().with_tag(key, value);
    }

    pub fn with_extra(&mut self, key: String, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let json_value = python_to_json_value(value)?;
        self.inner = self.inner.clone().with_extra(key, json_value);
        Ok(())
    }

    pub fn add_tag(&mut self, key: String, value: String) {
        self.inner.tags.insert(key, value);
    }

    pub fn add_extra(&mut self, key: String, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let json_value = python_to_json_value(value)?;
        self.inner.extra.insert(key, json_value);
        Ok(())
    }

    #[getter]
    pub fn tags(&self) -> HashMap<String, String> {
        self.inner.tags.clone()
    }

    #[getter]
    pub fn extra(&self, py: Python) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        for (key, value) in &self.inner.extra {
            let py_value = json_value_to_python(py, value)?;
            dict.set_item(key, py_value)?;
        }
        Ok(dict.into())
    }
}

impl PyEvent {
    pub fn inner(&self) -> &Event {
        &self.inner
    }

    pub fn into_inner(self) -> Event {
        self.inner
    }
}

fn python_to_json_value(value: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if value.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(b) = value.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(i) = value.extract::<i64>() {
        Ok(serde_json::Value::Number(serde_json::Number::from(i)))
    } else if let Ok(f) = value.extract::<f64>() {
        Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0)),
        ))
    } else if let Ok(s) = value.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(list) = value.downcast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_json_value(&item)?);
        }
        Ok(serde_json::Value::Array(vec))
    } else if let Ok(dict) = value.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            map.insert(key_str, python_to_json_value(&value)?);
        }
        Ok(serde_json::Value::Object(map))
    } else {
        Ok(serde_json::Value::String(value.str()?.to_cow()?.to_string()))
    }
}

fn json_value_to_python(py: Python, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    use pyo3::types::{PyBool, PyFloat, PyInt, PyString};

    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => {
            let py_bool = PyBool::new(py, *b);
            Ok(<pyo3::Bound<'_, PyBool> as Clone>::clone(&py_bool).into_any().unbind())
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                let py_int = PyInt::new(py, i);
                Ok(<pyo3::Bound<'_, PyInt> as Clone>::clone(&py_int).into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                let py_float = PyFloat::new(py, f);
                Ok(<pyo3::Bound<'_, PyFloat> as Clone>::clone(&py_float).into_any().unbind())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => {
            let py_str = PyString::new(py, s);
            Ok(<pyo3::Bound<'_, PyString> as Clone>::clone(&py_str).into_any().unbind())
        }
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_python(py, item)?)?;
            }
            Ok(list.into())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (key, value) in obj {
                dict.set_item(key, json_value_to_python(py, value)?)?;
            }
            Ok(dict.into())
        }
    }
}