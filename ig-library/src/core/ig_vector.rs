use std::sync::{Arc, RwLock};
use crate::core::memory::igMemory;

pub struct igVector<T>
where
    T: 'static + Send + Sync,
{
    data: Arc<RwLock<igMemory<T>>>,
    count: u64,
}

impl<T: Send + Sync> igVector<T> {
    pub fn new() -> Self {
        igVector {
            data: Arc::new(RwLock::new(igMemory::new())),
            count: 0,
        }
    }

    pub fn from_memory(data: Arc<RwLock<igMemory<T>>>, count: u64) -> Self {
        igVector {
            data,
            count,
        }
    }

    pub fn push(&mut self, val: T) {

    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    pub fn set(&self, index: usize, val: T) {
        self.data.write().unwrap().data[index] = val;
    }

    pub fn get_data(&self) -> Arc<RwLock<igMemory<T>>> {
        self.data.clone()
    }
}
