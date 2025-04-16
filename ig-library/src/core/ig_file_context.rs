use crate::core::fs::igFileWorkItemProcessor;
use crate::core::ig_archive::igArchive;
use phf::phf_map;
use std::sync::{Arc, Mutex};
use crate::core::ig_archive_manager::igArchiveManager;
use crate::core::ig_archive_mount_manager::igArchiveMountManager;
use crate::core::ig_std_lib_storage_device::igStdLibStorageDevice;

static VIRTUAL_DEVICES: phf::Map<&'static str, &'static str> = phf_map! {
    "actors"            => "actors",
    "anims"             => "anims",
    "behavior_events"   => "behavior_events",
    "animation_events"  => "animation_events",
    "behaviors"         => "behaviors",
    "cutscene"          => "cutscene",
    "data"              => "",
    "fonts"             => "fonts",
    "graphs"            => "graphs",
    "vsc"               => "vsc",
    "loosetextures"     => "loosetextures",
    "luts"              => "loosetextures/luts",
    "maps"              => "maps",
    "materials"         => "materialInstances",
    "models"            => "models",
    "motionpaths"       => "motionpaths",
    "renderer"          => "renderer",
    "scripts"           => "scripts",
    "shaders"           => "shaders",
    "sky"               => "sky",
    "sounds"            => "sounds",
    "spawnmeshes"       => "spawnmeshes",
    "textures"          => "textures",
    "ui"                => "ui",
    "vfx"               => "vfx",
    "cwd"               => "",
    "app"               => "",
};

pub struct igFileContext {
    _root: String,
    processor_stack: Arc<Mutex<dyn igFileWorkItemProcessor + Send + Sync>>
}

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

pub struct igFileWorkItem {
    pub path: String,
    pub flags: u32,
    pub work_type: WorkType,
}

impl igFileContext {
    pub fn open(&self, path: String, flags: u32) -> Self {
        todo!()
    }

    pub fn new(game_path: String) -> Self {
        let _root = game_path
            .trim_end_matches("\\")
            .trim_end_matches("/")
            .to_string();

        let processor_stack = igArchiveMountManager::new();
        { // Drop the lock as soon as possible
            let mut stack_lock = processor_stack.lock().unwrap();
            stack_lock.set_next_processor(igArchiveManager::new());
            stack_lock.set_next_processor(igStdLibStorageDevice::new());
        }

        igFileContext {
            _root,
            processor_stack
        }
    }

    pub fn initialize_update(&self, update_path: String) {
        let ig_arc = igArchive::open(self, update_path);
    }
}
