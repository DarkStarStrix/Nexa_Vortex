use pyo3::prelude::*;
use std::sync::Arc;

pub mod cpu_dispatch;
pub mod errors;
pub mod integrations;

use crate::cpu_dispatch::PyCpuDispatcher;
pub use integrations::VortexWorkQueue as RustVortexWorkQueue;

#[pyclass(name = "VortexWorkQueue")]
#[derive(Clone)]
pub struct PyVortexWorkQueue {
    inner: Arc<RustVortexWorkQueue<PyObject>>,
}

#[pymethods]
impl PyVortexWorkQueue {
    #[new]
    pub fn new(capacity: usize) -> PyResult<Self> {
        let queue = RustVortexWorkQueue::new(capacity)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(queue),
        })
    }

    pub fn push(&self, item: PyObject) -> PyResult<()> {
        self.inner
            .push(item)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    pub fn pop(&self) -> PyResult<Option<PyObject>> {
        Ok(self.inner.pop())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _vortex_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyVortexWorkQueue>()?;
    m.add_class::<PyCpuDispatcher>()?;
    Ok(())
}
