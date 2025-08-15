//! Kernel registration and management.
// This module provides a registry for compute kernels, allowing for dynamic
// loading and dispatch of kernels based on hardware capabilities.

use crate::errors::VortexError;
use std::collections::HashMap;

pub type KernelFn = fn(&[u8]) -> Vec<u8>;

/// Represents a compute kernel.
pub struct Kernel {
    pub name: String,
    pub implementation: KernelFn,
}

/// A registry for compute kernels.
pub struct KernelRegistry {
    kernels: HashMap<String, Kernel>,
}

impl KernelRegistry {
    /// Creates a new kernel registry.
    pub fn new() -> Self {
        KernelRegistry {
            kernels: HashMap::new(),
        }
    }

    /// Registers a new kernel.
    pub fn register(&mut self, kernel: Kernel) {
        self.kernels.insert(kernel.name.clone(), kernel);
    }

    /// Retrieves a kernel by name.
    pub fn get(&self, name: &str) -> Result<&Kernel, VortexError> {
        self.kernels
            .get(name)
            .ok_or_else(|| VortexError::KernelNotFound(name.to_string()))
    }
}
