use pyo3::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    _id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    // Sender is dropped, exit loop
                    break;
                }
            }
        });

        Worker { _id: id, thread }
    }
}

#[pyclass(name = "CpuDispatcher")]
pub struct PyCpuDispatcher {
    pool: Arc<ThreadPool>,
}

#[pymethods]
impl PyCpuDispatcher {
    #[new]
    fn new(num_threads: usize) -> Self {
        PyCpuDispatcher {
            pool: Arc::new(ThreadPool::new(num_threads)),
        }
    }

    pub fn dispatch(&self, callable: PyObject) -> PyResult<()> {
        let pool = self.pool.clone();
        pool.execute(move || {
            Python::with_gil(|py| {
                if let Err(e) = callable.call0(py) {
                    e.print(py);
                }
            });
        });
        Ok(())
    }
}

