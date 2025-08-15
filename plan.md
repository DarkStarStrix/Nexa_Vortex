# Nexa Vortex × Mesocarp — Full Integration Spec

This document is the authoritative, implementation-ready specification for integrating **Mesocarp** primitives into **Nexa Vortex**. It contains architecture, execution flow, Cargo/workspace layout, wrapper design, license handling, build instructions (so Mesocarp code is reused and included in `cargo build`), tests, CI, and a migration plan to upstream contributions.

> NOTE: This replaces the previous Vortex scaffold content with a focused integration spec. The repo scaffold still applies (see the `rust/vortex_core` and `python/vortex` layouts) — this doc expands the Rust core to vendor/use Mesocarp.

---

## 0. Goals & constraints

* **Primary goal:** reuse Mesocarp’s lock-free queues, broadcast, mailbox, timers, and journal as the concurrency/comm primitives inside Nexa Vortex’s Rust core to speed development, increase performance, and reduce risk.
* **Constraints:** Keep Mesocarp integration as an **optional** Cargo feature (`mesocarp_integration`) to manage LGPL obligations. Support two integration modes: **vendored** (path dependency included in repo/workspace) and **external** (git dependency installed during build). Default: vendored under `third_party/mesocarp` and integrated as a workspace member.

---

## 1. High-level design & mapping

### 1.1 Components to reuse from Mesocarp

* **WorkQueue / SPMC / SPSC** — CPU producer → GPU consumer pipelines; job dispatch. (`mesocarp::comms`)
* **Broadcast / Subscriber** — telemetry publishing and subscription. (`mesocarp::comms::broadcast`)
* **Mailbox / ThreadedMessenger** — control messages, pause/resume, mailbox routing. (`mesocarp::comms::mailbox`)
* **Hierarchical time wheel (HTW)** — scheduled timers for checkpoint/autotuner events. (`mesocarp::scheduling::htw`)
* **Journal / logging** — append-only telemetry persistence for long runs. (`mesocarp::logging::journal`)
* **Optional: GVT / sync primitives** — only if we later go multi-node/deterministic.

### 1.2 How Mesocarp fits into Nexa Vortex

* `vortex_core::cpu_dispatch` uses `WorkQueue` for enqueuing preprocessed batches.
* `vortex_core::telemetry` uses `Broadcast` to publish batch performance to subscribers in the same process. A small IPC bridge relays telemetry to Python.
* `vortex_core::controlplane` uses `Mailbox` to route control commands (pause, checkpoint, scale) between the Python control plane and internal Rust actors.
* `vortex_core::timers` replaces ad-hoc timers with `htw` for robust periodic events.
* `vortex_core::logging` uses `journal` as the primary runtime event store.

---

## 2. Licensing & distribution strategy

Mesocarp is LGPL-2.1 (as shipped). To avoid surprising downstream users and to keep Vortex's preferred license MIT/Apache, follow this plan:

1. **Optional feature flag** — add Cargo feature `mesocarp_integration`. Mesocarp code is included only when the feature is enabled. The Default build does not include Mesocarp, so Vortex core can remain permissively licensed. Document consequences in README.
2. **Vendored path dependency** — place Mesocarp under `third_party/mesocarp/` as a separate git submodule or a copy (if you own the code) and add it as a workspace member. That makes it obvious how to relink/replace Mesocarp and simplifies development. For distributions, document that enabling mesocarp\_integration requires compliance with LGPL obligations (source or relinkable object files offered).
3. Optionally, contact Mesocarp authors for a relicensing exception or dual license for Nexa Vortex if you want a permissive combined distribution.

Practical recommendation: default repo builds without Mesocarp. Developers who want Mesocarp set `--features mesocarp_integration` or `cargo build -p vortex_core --features mesocarp_integration` and ensure `third_party/mesocarp` exists.

---

## 3. Repo & Cargo workspace layout (concrete)

```
nexa-vortex/
├── Cargo.toml                 # workspace top-level
├── README.md
├── third_party/
│   └── mesocarp/              # git submodule or vendored copy
├── rust/
│   └── vortex_core/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── hw_profile.rs
│           ├── allocator.rs
│           ├── cpu_dispatch.rs
│           ├── kernel_registry.rs
│           ├── telemetry.rs
│           ├── controlplane.rs
│           ├── timers.rs
│           └── integrations/
│               └── mesocarp_wrapper.rs
└── python/
    └── vortex/
        ├── setup.py
        └── ...
```

Top-level `Cargo.toml` (workspace)

```toml
[workspace]
members = ["rust/vortex_core"]

[features]
mesocarp_integration = []
```

`rust/vortex_core/Cargo.toml` (key sections)

```toml
[package]
name = "vortex_core"
version = "0.1.0"
edition = "2021"

[lib]
name = "vortex_core"
crate-type = ["rlib","cdylib"]

[dependencies]
# common deps
serde = { version = "1.0", features = ["derive"] }
nvml-wrapper = "0.8"        # or nvml-sys binding for NVML
log = "0.4"
# Optional mesocarp
mesocarp = { path = "../../third_party/mesocarp", optional = true }

[features]
default = []
mesocarp_integration = ["mesocarp"]
```

Notes: Using `path` dependency points to vendored Mesocarp. If Mesocarp is a git repo we prefer adding it as a submodule at `third_party/mesocarp` so `cargo build` works.

---

## 4. mesocarp\_wrapper.rs — design and API

Place wrapper at `rust/vortex_core/src/integrations/mesocarp_wrapper.rs`. The wrapper *adapts* Mesocarp types to Vortex internal types and isolates Mesocarp-specific generics behind a small, stable API.

### 4.1 Goals for wrapper

* Hide Mesocarp internals from the rest of the codebase so Mesocarp can be swapped easily.
* Provide `VortexWorkQueue<T>`, `VortexBroadcast<T>`, `VortexMailbox` adapters.
* Use Rust `cfg(feature = "mesocarp_integration")` to conditionally compile the Mesocarp-backed implementations; provide fallback implementations for builds without Mesocarp.

### 4.2 Example wrapper (simplified)

```rust
// src/integrations/mesocarp_wrapper.rs
use crate::errors::VortexError;

#[cfg(feature = "mesocarp_integration")]
mod mesocarp_impl {
    use mesocarp::comms::{WorkQueue as MWorkQueue, Broadcast as MBroadcast};
    use std::sync::Arc;
    use crate::errors::VortexError;

    #[derive(Clone)]
    pub struct VortexWorkQueue<T> {
        inner: Arc<MWorkQueue<T>>,
    }

    impl<T> VortexWorkQueue<T> {
        pub fn new(capacity: usize) -> Result<Self, VortexError> {
            Ok(Self { inner: Arc::new(MWorkQueue::new(capacity).map_err(|e| VortexError::QueueCreation(e.to_string()))?) })
        }

        pub fn push(&self, item: T) -> Result<(), VortexError> { self.inner.push(item).map_err(|_| VortexError::Enqueue) }
        pub fn pop(&self) -> Option<T> { self.inner.try_pop() }
    }

    pub struct VortexBroadcast<T> { inner: Arc<MBroadcast<T>> }
    // ... similar
}

#[cfg(not(feature = "mesocarp_integration"))]
mod fallback_impl {
    use std::sync::{Mutex, Arc};
    use std::collections::VecDeque;
    use crate::errors::VortexError;

    #[derive(Clone)]
    pub struct VortexWorkQueue<T> { inner: Arc<Mutex<VecDeque<T>>> }
    impl<T> VortexWorkQueue<T> {
        pub fn new(_capacity: usize) -> Result<Self, VortexError> { Ok(Self { inner: Arc::new(Mutex::new(VecDeque::new())) }) }
        pub fn push(&self, it: T) -> Result<(), VortexError> { self.inner.lock().unwrap().push_back(it); Ok(()) }
        pub fn pop(&self) -> Option<T> { self.inner.lock().unwrap().pop_front() }
    }
}

// Re-export the appropriate API
#[cfg(feature = "mesocarp_integration")] pub use mesocarp_impl::*;
#[cfg(not(feature = "mesocarp_integration"))] pub use fallback_impl::*;
```

This pattern keeps the public API stable while letting Mesocarp plug in with `--features mesocarp_integration`.

---

## 5. Concrete usage points in Vortex code

* **cpu\_dispatch.rs** — create `VortexWorkQueue<Batch>` with capacity tuned to N (e.g., 1024). Producers (CPU preproc threads) call `push(batch)`; consumers (GPU launch threads) call `pop()` in a tight loop.
* **telemetry.rs** — construct `VortexBroadcast<Telemetry>` and register subscribers for Python bridge and internal autotuner.
* **controlplane.rs** — `VortexMailbox` routes commands from Python to worker threads using `Mailbox` primitives.
* **timers.rs** — use htw to schedule `checkpoint()` and `compaction()` events with millisecond resolution.

---

## 6. Build integration — how to include Mesocarp in `cargo build`

### 6.1 Vendored (recommended for development)

1. Add Mesocarp to repository: `git submodule add <mesocarp-repo-url> third_party/mesocarp` or copy Mesocarp repo into `third_party/mesocarp`.
2. Ensure `third_party/mesocarp/Cargo.toml` has a package name (e.g., `mesocarp`).
3. Add path dependency to `rust/vortex_core/Cargo.toml` as shown above.
4. Build with feature: `cargo build -p vortex_core --features mesocarp_integration`.

### 6.2 External git dependency (alternative)

* Use in Cargo.toml:

```toml
mesocarp = { git = "https://github.com/<user>/mesocarp.git", tag = "vX.Y.Z", optional = true }
```

* Note: If Mesocarp’s license or API changes, CI may break. Vendoring gives reproducibility.

---

## 7. Tests & CI

### 7.1 Unit tests

* `tests/integration_mesocarp_queue.rs` — starts a small threaded producer & consumer using `VortexWorkQueue` and asserts no lost items.
* `tests/telemetry_broadcast.rs` — publish telemetry messages and verify subscriber receives them.

`Cargo.toml` test scripts

```toml
[package.metadata.ci]
# run both features
scripts = [
  "cargo test -p vortex_core",
  "cargo test -p vortex_core --features mesocarp_integration"
]
```

### 7.2 CI (GitHub Actions)

* Matrix build: Rust stable, features: none; features: mesocarp\_integration (vendored is included via submodule checkout). Run `cargo test` for both.
* Additional job: run benchmarks (nightly) if you want microperf regression checks.

---

## 8. Migration & Contribution plan

1. **Phase 0**: Add Mesocarp as submodule, add wrapper, and replace internal toy queue with wrapper for CPU→GPU dispatch. Run tests without Mesocarp (fallback) and with Mesocarp (feature enabled) to validate parity.
2. **Phase 1**: Replace telemetry and timers with Mesocarp's broadcast and HTW. Add tests and benchmarks. Measure throughput and latency.
3. **Phase 2**: Replace logging with Mesocarp journal for persistence. Add CSV exporter to Python.
4. **Phase 3**: If desired, upstream contributions: PR missing improvements to Mesocarp (e.g., numeric metrics, backpressure policies) and request improved or dual licensing if required.

When submitting PRs into Nexa Vortex repo, include detailed notes about licensing and the optional feature gate.

---

## 9. Example: End-to-end flow using Mesocarp primitives (detailed)

1. Python bootstrap collects hardware profile and posts job descriptor to Rust `vortexd` over unix socket. `vortexd` initializes `VortexWorkQueue<Batch>` (mesocarp-backed) with capacity 2048.
2. A pool of CPU worker threads preprocess and push batches into the `VortexWorkQueue` (mesocarp's SPMC queue). Producers are pinned to NUMA cores close to corresponding GPUs.
3. GPU launcher threads pop batches from `VortexWorkQueue`, perform pinned-memory DMA or zero-copy, and call kernel launches. Each launcher publishes `Telemetry` events to `VortexBroadcast<Telemetry>`.
4. Python bridge subscribes to `VortexBroadcast` and receives telemetry asynchronously. The autotuner (Python) consumes telemetry and sends `ControlCommand` messages via `VortexMailbox` to adjust `batch_size` or `feed_rate`.
5. Periodic tasks (checkpoints, compaction) are scheduled using `htw` and invoke allocator compaction or checkpoint APIs.

This design keeps the critical queues and timers in Rust for low latency and uses Python only for high-level control and model handling.

---

## 10. Example code snippets & tests (practical)

### 10.1 Cargo workspace (top-level `Cargo.toml`)

```toml
[workspace]
members = ["rust/vortex_core"]

[profile.release]
opt-level = 3

[workspace.metadata]
# docs for maintainers
```

### 10.2 vortex\_core Cargo features (repeated)

```toml
[features]
default = []
mesocarp_integration = ["mesocarp"]
```

### 10.3 Simple test using wrapper (rust/vortex\_core/tests/queue\_integration.rs)

```rust
use vortex_core::integrations::VortexWorkQueue;
use std::thread;
use std::sync::Arc;

#[test]
fn smoke_queue() {
    let q = Arc::new(VortexWorkQueue::new(1024).unwrap());
    let producer = {
        let q = Arc::clone(&q);
        thread::spawn(move || {
            for i in 0..1000 { q.push(i).unwrap(); }
        })
    };
    let consumer = {
        let q = Arc::clone(&q);
        thread::spawn(move || {
            let mut seen = 0;
            while seen < 1000 {
                if let Some(v) = q.pop() {
                    seen += 1;
                }
            }
            assert_eq!(seen, 1000);
        })
    };
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

---

## Developer Build Checklist

1. Ensure Mesocarp is present at `third_party/mesocarp` (as a submodule or copy).
2. Run `cargo build -p vortex_core` for fallback (no Mesocarp).
3. Run `cargo build -p vortex_core --features mesocarp_integration` for Mesocarp-backed primitives.
4. Run tests: `cargo test -p vortex_core` and `cargo test -p vortex_core --features mesocarp_integration`.
5. For Python, build the extension with maturin or setuptools-rust as appropriate.
6. See `docs/licensing.md` for license compliance when distributing binaries.
