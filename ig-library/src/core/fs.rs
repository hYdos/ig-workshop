use crate::core::ig_file_context::igFileWorkItem;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

pub struct igFileDescriptor {
    pub _path: String,
    pub _position: u64,
    pub _size: u64,
    pub _device: Option<Arc<dyn igStorageDevice + Send + Sync>>,
    pub _handle: Option<Cursor<Vec<u8>>>,
    pub _flags: u32,
    pub _work_item_active_count: i32,
    pub endianness: Endian,
}

pub enum Endian {
    Big,
    Little,
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

pub trait igFileWorkItemProcessor {
    fn process(&self, work_item: igFileWorkItem);
    /// Allows setting the next processor in the chain. If the next processor is already set and this is called,  pass this to the next processor to process its next processor
    fn set_next_processor(&mut self, processor: Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>);
    fn send_to_next_processor(&self, work_item: igFileWorkItem);
}

pub trait igStorageDevice: igFileWorkItemProcessor {
    fn exists(self: Self, work_item: igFileWorkItem);
    fn open(self: Self, work_item: igFileWorkItem);
    fn close(self: Self, work_item: igFileWorkItem);
    fn read(self: Self, work_item: igFileWorkItem);
    fn write(self: Self, work_item: igFileWorkItem);
    fn truncate(self: Self, work_item: igFileWorkItem);
    fn mkdir(self: Self, work_item: igFileWorkItem);
    fn rmdir(self: Self, work_item: igFileWorkItem);
    fn get_file_list(self: Self, work_item: igFileWorkItem);
    fn get_file_list_with_sizes(self: Self, work_item: igFileWorkItem);
    fn unlink(self: Self, work_item: igFileWorkItem);
    fn rename(self: Self, work_item: igFileWorkItem);
    fn prefetch(self: Self, work_item: igFileWorkItem);
    fn format(self: Self, work_item: igFileWorkItem);
    fn commit(self: Self, work_item: igFileWorkItem);
}
