pub mod allocator;
pub mod cpu_dispatch;
pub mod errors;
pub mod hw_profile;
pub mod integrations;
pub mod kernel_registry;
pub mod telemetry;

use crate::hw_profile::{detect_hardware, HardwareProfile};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub headers: HashMap<String, String>,
    #[pyo3(get)]
    pub payload: Vec<u8>,
}

#[pymethods]
impl Message {
    #[new]
    fn new(headers: HashMap<String, String>, payload: Vec<u8>) -> Self {
        Message {
            id: Uuid::new_v4().to_string(),
            headers,
            payload,
        }
    }

    #[staticmethod]
    fn from_dict(py: Python, data: &PyDict) -> PyResult<Self> {
        let headers: HashMap<String, String> = data
            .get_item("headers")
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'headers'"))?
            .extract()?;

        let message = Message {
            id: Uuid::new_v4().to_string(),
            headers,
            payload: data
                .get_item("payload")
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'payload'"))?
                .extract()?,
        };
        Ok(message)
    }
}

#[pymodule]
fn vortex_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Message>()?;
    m.add_class::<HardwareProfile>()?;
    m.add_function(wrap_pyfunction!(detect_hardware, m)?)?;
    Ok(())
}
