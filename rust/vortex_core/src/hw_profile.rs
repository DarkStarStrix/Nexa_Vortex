use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, System, SystemExt};

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareProfile {
    #[pyo3(get)]
    pub cpu_cores: u32,
    #[pyo3(get)]
    pub total_memory_gb: u64,
    #[pyo3(get)]
    pub cpu_model: String,
    #[pyo3(get)]
    pub cpu_vendor: String,
    #[pyo3(get)]
    pub cpu_frequency_mhz: u64,
}

#[pymethods]
impl HardwareProfile {
    #[new]
    pub fn py_new(
        cpu_cores: u32,
        total_memory_gb: u64,
        cpu_model: String,
        cpu_vendor: String,
        cpu_frequency_mhz: u64,
    ) -> Self {
        Self {
            cpu_cores,
            total_memory_gb,
            cpu_model,
            cpu_vendor,
            cpu_frequency_mhz,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "HardwareProfile(cpu_cores={}, cpu_model='{}', memory_gb={})",
            self.cpu_cores, self.cpu_model, self.total_memory_gb
        )
    }
}

#[pyfunction]
pub fn detect_hardware() -> HardwareProfile {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_cores = sys.physical_core_count().unwrap_or(0) as u32;
    let total_memory_gb = sys.total_memory() / (1024 * 1024 * 1024);
    let first_cpu = sys.cpus().get(0);
    let cpu_model = first_cpu.map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
    let cpu_vendor = first_cpu.map_or("Unknown".to_string(), |cpu| cpu.vendor_id().to_string());
    let cpu_frequency_mhz = first_cpu.map_or(0, |cpu| cpu.frequency());

    HardwareProfile {
        cpu_cores,
        total_memory_gb,
        cpu_model,
        cpu_vendor,
        cpu_frequency_mhz,
    }
}
