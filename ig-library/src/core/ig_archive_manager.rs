use crate::core::fs::igFileWorkItemProcessor;
use crate::core::ig_file_context::igFileWorkItem;
use std::sync::{Arc, Mutex};
use crate::core::ig_lists::igArchiveList;

pub struct igArchiveManager {
    next_processor: Option<Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>>,
    _archive_list: igArchiveList,
    _patch_archives: igArchiveList
}

impl igArchiveManager {
    pub fn new() -> Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>> {
        Arc::new(Mutex::new(Self {
            next_processor: None,
            _archive_list: igArchiveList::new(),
            _patch_archives: igArchiveList::new(),
        }))
    }
}

impl igFileWorkItemProcessor for igArchiveManager {

    fn process(&self, work_item: igFileWorkItem) {
        
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
