use std::sync::{Arc, Mutex, RwLock};
use crate::core::ig_fs::{igFileWorkItemProcessor, igStorageDevice};
use crate::core::ig_file_context::igFileWorkItem;

/// In igWorkshop, this type is not too useful and mainly exists for parity between igAlchemy, and igCauldron
pub struct igArchiveMountManager {
    next_processor: Option<Arc<RwLock<dyn igFileWorkItemProcessor>>>
}

impl igArchiveMountManager {
    pub fn new() -> Arc<Mutex<dyn igFileWorkItemProcessor>> {
        Arc::new(Mutex::new(Self {
            next_processor: None,
        }))
    }
}

impl igFileWorkItemProcessor for igArchiveMountManager {
    
    fn process(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        self.send_to_next_processor(this, work_item);
    }

    fn set_next_processor(&mut self, new_processor: Arc<RwLock<dyn igFileWorkItemProcessor>>) {
        if let Some(next_processor) = &self.next_processor {
            if let Ok(mut processor) = next_processor.write() {
                processor.set_next_processor(new_processor);
                return;
            }
        }
        self.next_processor = Some(new_processor);
    }

    fn send_to_next_processor(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        if let Some(processor) = &self.next_processor {
            let processor_lock = processor.read().unwrap();
            processor_lock.process(this, work_item);
        }
    }

    fn as_ig_storage(&self) -> &dyn igStorageDevice {
        panic!("Tried getting igArchiveMountManager as igStorage")
    }
}