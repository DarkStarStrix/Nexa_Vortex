# Nexa Vortex Project Documentation

## 1. Overview

Nexa Vortex is a high-performance computing library designed to create scalable, hardware-agnostic data processing pipelines, particularly for machine learning and scientific computing workloads. It leverages a hybrid Python/Rust architecture to combine ease of use and high-level control with low-level performance and resource management.

The core philosophy is to abstract away the complexities of hardware-specific optimizations, concurrency, and memory management, allowing developers to focus on the logic of their data pipelines.

## 2. Core Architecture

The library is built on three pillars:

1.  **Mesocarp Integration (Optional)**: A foundational layer providing high-performance, battle-tested primitives for concurrency, synchronization, and communication. It is integrated via a feature flag (`mesocarp_integration`) to avoid license compatibility issues (LGPL-2.1) unless explicitly opted in. When not used, the library provides fallback implementations using the standard library.

2.  **Intelligence Layer (Python)**: This is the user-facing API where developers define and control their data pipelines. It is responsible for:
    *   **Telemetry**: Gathering real-time metrics from the system (e.g., CPU/GPU utilization, memory usage, queue saturation) using interfaces like SMI (System Management Interface).
    *   **Control Plane**: Making intelligent, dynamic decisions based on telemetry data. This includes adjusting thread allocation, batch sizes, and managing the flow of data to optimize performance and resource utilization.

3.  **Execution Layer (Rust)**: This is the high-performance core that runs the actual compute workloads. It is exposed to Python via PyO3 bindings. Its responsibilities include:
    *   **CPU Pre-processing**: Efficiently handling tasks like data loading, cleaning, scaling, and batching. It uses a CPU dispatcher (thread pool) to parallelize this work across available cores.
    *   **GPU Kernel Management**: Launching and profiling GPU kernels for massively parallel computation (e.g., matrix multiplication, weight updates in ML).
    *   **Asynchronous Pipeline**: Managing the flow of data from CPU to GPU in an asynchronous "conveyor belt" fashion. Data chunks are prepared on the CPU and fed to the GPU, ensuring the GPU is always saturated with work while managing memory efficiently.
    *   **Dynamic Resource Management**: Adjusting resources at runtime based on commands from the control plane to maintain optimal performance.

## 3. Logic and Data Flow

The typical data flow through a Nexa Vortex pipeline is as follows:

1.  **Initialization**: The Python control plane is initialized. It starts the telemetry manager and sets up the Rust core, including the CPU dispatcher and work queues, based on the detected hardware (e.g., number of CPU cores).

2.  **Data Ingestion**: Raw data is loaded into the system.

3.  **CPU Pre-processing**: The control plane dispatches pre-processing tasks to the Rust `CpuDispatcher`. The raw data is partitioned and processed in parallel by a pool of CPU-bound threads. This includes cleaning, normalization, and transformation.

4.  **Batching**: The processed data is packaged into standardized "batches" or "chunks." These chunks are placed into a high-performance, thread-safe work queue (`VortexWorkQueue`).

5.  **CPU-to-GPU Transfer**: A dedicated thread (or threads) pulls data batches from the work queue and transfers them to the GPU's memory. This process is asynchronous. The goal is to feed the GPU stack-by-stack until it is full, ensuring the compute units are never idle.

6.  **GPU Execution**: Once data is on the GPU, the Rust core launches the appropriate compute kernels. The GPU performs intensive computations (e.g., model training forward/backward pass).

7.  **Asynchronous "Conveyor Belt"**: While the GPU is busy, the CPU continues to pre-process the next set of data batches. As soon as the GPU completes a task, the next batch is ready. This creates a smooth, asynchronous pipeline that maximizes hardware saturation.

8.  **Dynamic Optimization**: Throughout this process, the telemetry layer monitors system performance. The Python control plane analyzes this data and can issue commands to the Rust core at runtime, such as:
    *   Increasing or decreasing the number of CPU threads.
    *   Adjusting the size of data batches to balance latency and throughput.
    *   Pausing or throttling data transfer to manage memory pressure.

This closed-loop system ensures the pipeline is not just fast but also adaptive and efficient across different hardware configurations and workloads.

