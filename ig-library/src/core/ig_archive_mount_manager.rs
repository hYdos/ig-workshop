use std::sync::{Arc, Mutex};
use crate::core::fs::igFileWorkItemProcessor;
use crate::core::ig_file_context::igFileWorkItem;

/// In igToolbox, this type is not too useful and mainly exists for parity between igAlchemy, and igCauldron
pub struct igArchiveMountManager {
    next_processor: Option<Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>>
}

impl igArchiveMountManager {
    pub fn new() -> Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>> {
        Arc::new(Mutex::new(Self {
            next_processor: None,
        }))
    }
}

impl igFileWorkItemProcessor for igArchiveMountManager {
    
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