use crate::core::ig_fs::{igFileWorkItemProcessor, igStorageDevice};
use crate::core::ig_archive::igArchive;
use crate::core::ig_file_context::WorkStatus::kStatusComplete;
use crate::core::ig_file_context::{igFileContext, igFileWorkItem, WorkType};
use crate::core::ig_lists::igArchiveList;
use crate::core::ig_registry::igRegistry;
use crate::util::ig_hash;
use std::sync::{Arc, Mutex, RwLock};

pub struct igArchiveManager {
    next_processor: Option<Arc<RwLock<dyn igFileWorkItemProcessor>>>,
    pub _archive_list: igArchiveList,
    pub _patch_archives: igArchiveList,
}

impl igArchiveManager {
    pub fn new() -> Arc<RwLock<igArchiveManager>> {
        Arc::new(RwLock::new(Self {
            next_processor: None,
            _archive_list: igArchiveList::new(),
            _patch_archives: igArchiveList::new(),
        }))
    }


    /// Loads an archive from the given path.
    pub fn load_archive(
        archive_manager: Arc<RwLock<igArchiveManager>>,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        path: String,
    ) -> Arc<igArchive> {
        if let Ok(archive_manager) = archive_manager.read() {
            if let Some(archive) = archive_manager.try_get_archive(&path) {
                return archive
            }
        }

        let arc = Arc::new(igArchive::open(ig_file_context, ig_registry, &path).unwrap());

        if let Ok(mut archive_manager) = archive_manager.write() {
            archive_manager._archive_list.push(arc.clone());
        }

        arc
    }

    pub fn try_get_archive(&self, path: &str) -> Option<Arc<igArchive>> {
        for arc in &self._archive_list {
            if arc._path.to_lowercase() == path.to_lowercase() {
                return Some(arc.clone());
            }
        }

        None
    }
}

impl igFileWorkItemProcessor for igArchiveManager {
    fn process(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        match work_item.work_type {
            WorkType::kTypeFileList => {
                let hash = ig_hash::hash(&work_item._path);
                for patch_archive in &self._patch_archives {
                    if ig_hash::hash(&patch_archive._path) == hash {
                        igStorageDevice::process(&patch_archive, this.clone(), work_item);
                        return;
                    }
                }
                for archive in &self._archive_list {
                    if ig_hash::hash(&archive._path) == hash {
                        igStorageDevice::process(&archive, this.clone(), work_item);
                        return;
                    }
                }
            }
            WorkType::kTypeInvalid => {
                self.send_to_next_processor(this, work_item);
                return;
            }
            _ => {
                for patch_archive in &self._patch_archives {
                    igStorageDevice::process(&patch_archive, this.clone(), work_item);
                    if work_item._status == kStatusComplete {
                        return;
                    }
                }
                for archive in &self._archive_list {
                    igStorageDevice::process(&archive, this.clone(), work_item);
                    if work_item._status == kStatusComplete {
                        return;
                    }
                }
            }
        }

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

    fn send_to_next_processor(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        if let Some(processor) = self.next_processor.clone() {
            let processor_lock = processor.read().unwrap();
            processor_lock.process(this, work_item);
        }
    }

    fn as_ig_storage(&self) -> &dyn igStorageDevice {
        panic!("Tried getting igArchiveManager as igStorage")
    }
}
