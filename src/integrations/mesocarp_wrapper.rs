use crate::errors::VortexError;

#[cfg(feature = "mesocarp_integration")]
mod mesocarp_impl {
    use crate::errors::VortexError;
    use mesocarp::sync::spsc::WorkQueue as MWorkQueue;
    use std::sync::Arc;

    #[derive(Clone)]
    pub struct VortexWorkQueue<T: Send> {
        inner: Arc<MWorkQueue<T>>,
    }

    impl<T: Send> VortexWorkQueue<T> {
        pub fn new(capacity: usize) -> Result<Self, VortexError> {
            Ok(Self {
                inner: Arc::new(MWorkQueue::new(capacity)),
            })
        }

        pub fn push(&self, item: T) -> Result<(), VortexError> {
            self.inner.push(item).map_err(|_| VortexError::Enqueue)
        }

        pub fn pop(&self) -> Option<T> {
            self.inner.try_pop()
        }
    }
}

#[cfg(not(feature = "mesocarp_integration"))]
mod fallback_impl {
    use crate::errors::VortexError;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    pub struct VortexWorkQueue<T> {
        inner: Arc<Mutex<VecDeque<T>>>,
    }

    impl<T> VortexWorkQueue<T> {
        pub fn new(_capacity: usize) -> Result<Self, VortexError> {
            Ok(Self {
                inner: Arc::new(Mutex::new(VecDeque::new())),
            })
        }

        pub fn push(&self, it: T) -> Result<(), VortexError> {
            self.inner.lock().unwrap().push_back(it);
            Ok(())
        }

        pub fn pop(&self) -> Option<T> {
            self.inner.lock().unwrap().pop_front()
        }
    }
}

#[cfg(feature = "mesocarp_integration")]
pub use mesocarp_impl::*;
#[cfg(not(feature = "mesocarp_integration"))]
pub use fallback_impl::*;

