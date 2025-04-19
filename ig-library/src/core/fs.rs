use crate::core::ig_file_context::{igFileWorkItem, WorkType};
use log::error;
use std::io::Cursor;
use std::sync::{Arc, Mutex, RwLock};

pub struct igFileDescriptor {
    pub _path: String,
    pub _position: u64,
    pub _size: u64,
    /// When this is used, it can safely be cast to a [igStorageDevice]
    pub _device: Option<Arc<Mutex<dyn igFileWorkItemProcessor>>>,
    // Sacrificing memory for simplicity. Could be using a Cursor<File> here. Tried union but looks like a mess. need a good rust solution here...
    pub _handle: Option<Cursor<Vec<u8>>>,
    pub _flags: u32,
    pub _work_item_active_count: i32,
    /// Exists only as a utility for reading. Does not exist in VV Alchemy
    pub endianness: Endian,
}

pub enum Endian {
    Big,
    Little,
    /// Should NEVER be used. Only exists for error checking
    Unknown,
}

impl igFileDescriptor {
    pub fn empty() -> Self {
        Self {
            _path: "".to_string(),
            _position: 0,
            _size: 0,
            _device: None,
            _handle: None,
            _flags: 0,
            _work_item_active_count: 0,
            endianness: Endian::Little,
        }
    }

    pub fn new(data: Cursor<Vec<u8>>, path: String, endianness: Endian) -> Self {
        Self {
            _path: path,
            _position: 0,
            _size: 0,
            _device: None,
            _handle: Some(data),
            _flags: 0,
            _work_item_active_count: 0,
            endianness,
        }
    }
}

pub trait igFileWorkItemProcessor: Send + Sync {
    fn process(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        _work_item: &mut igFileWorkItem,
    ) {
        panic!("Missing igFileWorkItemProcessor::process implementation")
    }

    /// Allows setting the next processor in the chain. If the next processor is already set and this is called,  pass this to the next processor to process its next processor
    fn set_next_processor(&mut self, processor: Arc<RwLock<dyn igFileWorkItemProcessor>>);
    fn send_to_next_processor(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    );

    fn as_ig_storage(&self) -> &dyn igStorageDevice;
}

pub trait igStorageDevice: igFileWorkItemProcessor {
    /// Will give a cloned result of the path stored in the device.
    fn get_path(&self) -> String;
    /// Will give a cloned result of the name of the device.
    fn get_name(&self) -> String;

    fn process(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        match work_item.work_type {
            WorkType::kTypeExists => igStorageDevice::exists(self, this, work_item),
            WorkType::kTypeOpen => igStorageDevice::open(self, this, work_item),
            WorkType::kTypeClose => igStorageDevice::close(self, this, work_item),
            WorkType::kTypeRead => igStorageDevice::read(self, this, work_item),
            WorkType::kTypeWrite => igStorageDevice::write(self, this, work_item),
            WorkType::kTypeTruncate => igStorageDevice::truncate(self, this, work_item),
            WorkType::kTypeMkdir => igStorageDevice::mkdir(self, this, work_item),
            WorkType::kTypeRmdir => igStorageDevice::rmdir(self, this, work_item),
            WorkType::kTypeFileList => igStorageDevice::get_file_list(self, this, work_item),
            WorkType::kTypeFileListWithSizes => {
                igStorageDevice::get_file_list_with_sizes(self, this, work_item)
            }
            WorkType::kTypeUnlink => igStorageDevice::unlink(self, this, work_item),
            WorkType::kTypeRename => igStorageDevice::rename(self, this, work_item),
            WorkType::kTypePrefetch => igStorageDevice::prefetch(self, this, work_item),
            WorkType::kTypeFormat => igStorageDevice::format(self, this, work_item),
            WorkType::kTypeCommit => igStorageDevice::commit(self, this, work_item),
            _ => {
                error!("Work type {:?} recognised", work_item.work_type);
            }
        }
    }

    fn exists(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn open(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn close(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn read(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn write(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn truncate(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    );
    fn mkdir(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn rmdir(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn get_file_list(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    );
    fn get_file_list_with_sizes(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    );
    fn unlink(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn rename(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn prefetch(
        &self,
        this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    );
    fn format(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
    fn commit(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem);
}
