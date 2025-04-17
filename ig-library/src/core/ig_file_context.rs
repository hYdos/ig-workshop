use crate::core::fs::{igFileDescriptor, igFileWorkItemProcessor, Endian};
use crate::core::ig_archive::igArchive;
use crate::core::ig_archive_manager::igArchiveManager;
use crate::core::ig_archive_mount_manager::igArchiveMountManager;
use crate::core::ig_file_context::WorkItemBuffer::Invalid;
use crate::core::ig_registry::igRegistry;
use crate::core::ig_std_lib_storage_device::igStdLibStorageDevice;
use log::{debug, error};
use std::sync::{Arc, Mutex};
// use phf::phf_map;

//
// static VIRTUAL_DEVICES: phf::Map<&'static str, &'static str> = phf_map! {
//     "actors"            => "actors",
//     "anims"             => "anims",
//     "behavior_events"   => "behavior_events",
//     "animation_events"  => "animation_events",
//     "behaviors"         => "behaviors",
//     "cutscene"          => "cutscene",
//     "data"              => "",
//     "fonts"             => "fonts",
//     "graphs"            => "graphs",
//     "vsc"               => "vsc",
//     "loosetextures"     => "loosetextures",
//     "luts"              => "loosetextures/luts",
//     "maps"              => "maps",
//     "materials"         => "materialInstances",
//     "models"            => "models",
//     "motionpaths"       => "motionpaths",
//     "renderer"          => "renderer",
//     "scripts"           => "scripts",
//     "shaders"           => "shaders",
//     "sky"               => "sky",
//     "sounds"            => "sounds",
//     "spawnmeshes"       => "spawnmeshes",
//     "textures"          => "textures",
//     "ui"                => "ui",
//     "vfx"               => "vfx",
//     "cwd"               => "",
//     "app"               => "",
// };

pub struct igFileContext {
    pub _root: String,
    archive_manager: Arc<Mutex<igArchiveManager>>,
    processor_stack: Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>,
}

#[derive(Debug)]
pub enum WorkType {
    kTypeInvalid = 0,
    kTypeExists = 1,
    kTypeOpen = 2,
    kTypeClose = 3,
    kTypeRead = 4,
    kTypeWrite = 5,
    kTypeTruncate = 6,
    kTypeMkdir = 7,
    kTypeRmdir = 8,
    kTypeFileList = 9,
    kTypeFileListWithSizes = 10,
    kTypeUnlink = 11,
    kTypeRename = 12,
    kTypePrefetch = 13,
    kTypeFormat = 14,
    kTypeCommit = 15,
}

#[derive(Debug, PartialEq)]
pub enum WorkStatus {
    kStatusInactive,
    kStatusActive,
    kStatusComplete,
    kStatusDeviceNotFound,
    kStatusInvalidPath,
    kStatusTooManyOpenFiles,
    kStatusBadParam,
    kStatusOutOfMemory,
    kStatusDiskFull,
    kStatusDoorOpen,
    kStatusReadError,
    kStatusWriteError,
    kStatusAlreadyInUse,
    kStatusAlreadyExists,
    kStatusEndOfFile,
    kStatusDeviceNotInitialized,
    kStatusMediaUnformatted,
    kStatusMediaCorrupt,
    kStatusPermissionDenied,
    kStatusGeneralError,
    kStatusStopped,
    kStatusUnsupported,
}

pub enum WorkItemBuffer {
    /// Not a reference list. This is owned, But named like this to match Alchemy's igStringRefList
    StringRefList(Vec<String>),
    Bytes(Vec<u8>),
    Invalid(),
}

pub struct igFileWorkItem<'a> {
    /// The current [igFileContext] for this Alchemy instance
    pub file_context: &'a igFileContext,
    /// The current [igRegistry] for this Alchemy instance
    pub ig_registry: &'a igRegistry,
    /// The (usual) result after processing a igFileWorkItem
    pub _file: igFileDescriptor,
    /// The path to the file
    pub _path: String,
    /// Flags for opening the file
    pub flags: u32,
    /// The type of work to be completed
    pub work_type: WorkType,
    /// Allows returning a status after a job has been completed
    pub _status: WorkStatus,
    /// used on igStorage read()
    pub _offset: u64,
    /// used on igStorage read()
    pub _buffer: WorkItemBuffer,
}

impl igFileContext {
    pub fn open(&self, ig_registry: &igRegistry, path: String, flags: u32) -> igFileDescriptor {
        debug!("Opening path \"{}\"", path);
        let mut work_item = igFileWorkItem {
            file_context: &self,
            ig_registry,
            _file: igFileDescriptor {
                _path: path.clone(),
                _position: 0,
                _size: 0,
                _device: None,
                _handle: None,
                _flags: 0,
                _work_item_active_count: 0,
                endianness: Endian::Unknown,
            },
            _path: path,
            flags,
            work_type: WorkType::kTypeOpen,
            _status: WorkStatus::kStatusActive,
            _offset: 0,
            _buffer: Invalid(),
        };
        let processor_stack = self.processor_stack.lock().unwrap();
        processor_stack.process(self.processor_stack.clone(), &mut work_item);

        work_item._file
    }

    pub fn new(game_path: String) -> Self {
        let _root = game_path
            .trim_end_matches("\\")
            .trim_end_matches("/")
            .to_string();

        let archive_manager = igArchiveManager::new();

        let processor_stack = igArchiveMountManager::new();
        {
            // Drop the lock as soon as possible
            let mut stack_lock = processor_stack.lock().unwrap();
            stack_lock.set_next_processor(archive_manager.clone());
            stack_lock.set_next_processor(igStdLibStorageDevice::new());
        }

        igFileContext {
            _root,
            archive_manager,
            processor_stack,
        }
    }

    pub fn initialize_update(&self, ig_registry: &igRegistry, update_path: String) {
        let load_update_result = igArchive::open(self, ig_registry, update_path);
        if let Ok(update_pak) = load_update_result {
            if let Ok(mut archive_manager) = self.archive_manager.lock() {
                archive_manager._patch_archives.push(update_pak);
            }
        } else {
            error!(
                "Failed to load update.pak: {}",
                load_update_result.err().unwrap()
            )
        }
    }
}
