//! Memory allocation and management.
// This module will handle memory allocation, including page-aware and NUMA-aware
// allocation strategies to optimize data locality and performance.

use std::alloc::{alloc, dealloc, Layout, LayoutError};

/// A simple memory allocator.
pub struct Allocator;

impl Allocator {
    /// Creates a new allocator.
    pub fn new() -> Self {
        Allocator
    }

    /// Allocates a block of memory with the given size and alignment.
    pub fn allocate(&self, size: usize, align: usize) -> Result<*mut u8, LayoutError> {
        if size == 0 {
            return Ok(std::ptr::null_mut());
        }
        unsafe {
            let layout = Layout::from_size_align(size, align)?;
            let ptr = alloc(layout);
            if ptr.is_null() {
                // In a real scenario, handle allocation failure.
                // For now, we rely on the layout error for panicking.
                // A more robust implementation would return a custom error.
            }
            Ok(ptr)
        }
    }

    /// Deallocates a block of memory.
    pub fn deallocate(&self, ptr: *mut u8, size: usize, align: usize) {
        if !ptr.is_null() && size > 0 {
            unsafe {
                if let Ok(layout) = Layout::from_size_align(size, align) {
                    dealloc(ptr, layout);
                }
            }
        }
    }
}
