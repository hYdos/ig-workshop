use crate::core::fs::igFileWorkItemProcessor;
use crate::core::ig_file_context::igFileWorkItem;
use std::sync::{Arc, Mutex};

/// This struct is shared across any device using rust's standard library. In igCauldron, this type is most similar to igWin32StorageDevice
pub struct igStdLibStorageDevice {
    next_processor: Option<Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>>,
}

impl igStdLibStorageDevice {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            next_processor: None,
        }))
    }
}

impl igFileWorkItemProcessor for igStdLibStorageDevice {

    fn process(&self, work_item: igFileWorkItem) {
        self.send_to_next_processor(work_item);
    }

    fn set_next_processor(&mut self, processor: Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>) {
        self.next_processor = Some(processor);
    }

    fn send_to_next_processor(&self, work_item: igFileWorkItem) {
        if let Some(processor) = self.next_processor.clone() {
            let processor_lock = processor.lock().unwrap();
            processor_lock.process(work_item);
        }
    }
}
