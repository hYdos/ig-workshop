use crate::core::fs::{igFileWorkItemProcessor, igStorageDevice};
use crate::core::ig_file_context::WorkStatus::*;
use crate::core::ig_file_context::{igFileWorkItem, WorkItemBuffer};
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use walkdir::WalkDir;

/// This struct is shared across any device using rust's standard library. In igCauldron, this type is most similar to igWin32StorageDevice
pub struct igStdLibStorageDevice {
    _path: String,
    _name: String,
    next_processor: Option<Arc<RwLock<dyn igFileWorkItemProcessor>>>,
}

impl igStdLibStorageDevice {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            _path: "".to_string(),
            _name: "".to_string(),
            next_processor: None,
        }))
    }
}

impl igStdLibStorageDevice {
    fn get_combined_path(&self, work_item: &mut igFileWorkItem) -> String {
        // igCauldron accidentally did an if check here for no reason. Unsure as to why.
        PathBuf::from(&work_item.file_context._root)
            .join(&work_item._path)
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl igStorageDevice for igStdLibStorageDevice {
    fn get_path(&self) -> String {
        self._path.clone()
    }

    fn get_name(&self) -> String {
        self._name.clone()
    }

    fn exists(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        let full_path = self.get_combined_path(work_item);
        if Path::exists(full_path.as_ref()) {
            work_item._status = kStatusComplete
        } else {
            work_item._status = kStatusInvalidPath
        }
    }

    fn open(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        let full_path = self.get_combined_path(work_item);
        let result = File::open(full_path);
        if result.is_ok() {
            let mut buffer = Vec::new();
            result.unwrap().read_to_end(&mut buffer).unwrap();

            work_item._file._device = Some(this);
            work_item._file._handle = Some(Cursor::new(buffer));
            work_item._status = kStatusComplete;
        } else {
            let error = result.err().unwrap();
            match error.kind() {
                _ => {}
            }
        }
    }

    fn close(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._file._handle = None;
    }

    // This implementation is strange (but from igCauldron, so I don't think it's wrong).
    // It seems to write inside the read function, but the write function is unsupported. Got to talk to jasleen about this at some point
    fn read(&self, _this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        let file_descriptor = &mut work_item._file;

        match &work_item._buffer {
            WorkItemBuffer::Bytes(bytes) => {
                if let Some(handle) = &mut file_descriptor._handle {
                    let initial_offset = handle.position();

                    handle.seek(SeekFrom::Start(work_item._offset)).unwrap();
                    if handle.write(bytes).is_err() {
                        work_item._status = kStatusGeneralError;
                        return;
                    }

                    handle.seek(SeekFrom::Start(initial_offset)).unwrap();
                } else {
                    work_item._status = kStatusStopped;
                }
            }
            _ => {
                work_item._status = kStatusGeneralError;
                return;
            }
        }
    }

    fn write(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn truncate(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn mkdir(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn rmdir(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        let full_path = self.get_combined_path(work_item);
        let result = fs::remove_dir_all(full_path);
        if result.is_err() {
            work_item._status = kStatusUnsupported;
        } else {
            work_item._status = kStatusComplete
        }
    }

    fn get_file_list(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        match &mut work_item._buffer {
            WorkItemBuffer::StringRefList(directory_list) => {
                for entry in WalkDir::new(&work_item._path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_file())
                {
                    directory_list.push(entry.path().to_string_lossy().into_owned());
                }

                work_item._status = kStatusComplete;
            }
            _ => {
                work_item._status = kStatusGeneralError;
                return;
            }
        }
    }

    fn get_file_list_with_sizes(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn unlink(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn rename(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn prefetch(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn format(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn commit(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }
}

impl igFileWorkItemProcessor for igStdLibStorageDevice {
    fn process(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        igStorageDevice::process(self, this, work_item);
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
        self
    }
}
