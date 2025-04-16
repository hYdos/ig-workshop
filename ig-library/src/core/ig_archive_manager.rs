use crate::core::fs::{igFileWorkItemProcessor, igStorageDevice};
use crate::core::ig_file_context::{igFileWorkItem, WorkType};
use crate::core::ig_lists::igArchiveList;
use crate::core::util::ig_hash;
use std::sync::{Arc, Mutex};

pub struct igArchiveManager {
    next_processor: Option<Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>>,
    _archive_list: igArchiveList,
    _patch_archives: igArchiveList,
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
    fn process(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        match work_item.work_type {
            WorkType::kTypeFileList => {
                let hash = ig_hash::hash(&work_item._path);
                for patch_archive in &self._patch_archives.items {
                    if ig_hash::hash(&patch_archive._path) == hash {
                        todo!("patch_archive.process(work_item)");
                    }
                }
                for archive in &self._archive_list.items {
                    if ig_hash::hash(&archive._path) == hash {
                        todo!("archive.process(work_item)");
                    }
                }
            }
            WorkType::kTypeInvalid => return,
            _ => {}
        }
    }

    fn set_next_processor(&mut self, processor: Arc<Mutex<dyn igFileWorkItemProcessor>>) {
        self.next_processor = Some(processor);
    }

    fn send_to_next_processor(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        if let Some(processor) = self.next_processor.clone() {
            let processor_lock = processor.lock().unwrap();
            processor_lock.process(this, work_item);
        }
    }

    fn as_ig_storage(&self) -> &dyn igStorageDevice {
        panic!("Tried getting igArchiveManager as igStorage")
    }
}
