use crate::client::client::CClient;
use crate::core::ig_ark_core::igArkCore;
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_core_platform::IG_CORE_PLATFORM::*;
use crate::core::ig_external_ref::igExternalReferenceSystem;
use crate::core::ig_file_context::{get_native_path, igFileContext};
use crate::core::ig_handle::igObjectHandleManager;
use crate::core::ig_objects::{igObject, igObjectDirectory, igObjectStreamManager};
use crate::core::ig_registry::igRegistry;
use std::sync::{Arc, RwLock};
use crate::tfb::tfb_game::tfbApplication;
use crate::util::ig_hash::hash_lower;

/// Used as a placeholder where no value is used but one is needed
pub struct igNoValue;

/// After early initialization, this type becomes available to make getting state a less painful task
pub struct igAlchemy {
    pub ark_core: igArkCore,
    pub file_context: igFileContext,
    pub registry: igRegistry,
    pub object_stream_manager: igObjectStreamManager,
    pub ig_ext_ref_system: igExternalReferenceSystem,
    pub ig_object_handle_manager: igObjectHandleManager,
    /// Alchemy Laboratory Implementation
    pub client: CClient,
    /// TFBTool Implementation
    pub tfb_env: tfbApplication
}

impl igAlchemy {
    pub fn new(
        ig_file_context: igFileContext,
        ig_registry: igRegistry,
        ig_ark_core: igArkCore,
    ) -> igAlchemy {
        igAlchemy {
            ark_core: ig_ark_core,
            file_context: ig_file_context,
            object_stream_manager: igObjectStreamManager::new(),
            ig_ext_ref_system: igExternalReferenceSystem::new(),
            ig_object_handle_manager: igObjectHandleManager::new(),
            client: CClient::init(&ig_registry),
            registry: ig_registry,
            tfb_env: tfbApplication::open(),
        }
    }

    pub fn get_if_loaded(&self, path: &str) -> Option<Arc<RwLock<igObjectDirectory>>> {
        let file_path = get_native_path(path);
        let file_path_hash = hash_lower(&file_path);
        
        if self.object_stream_manager.path_to_directory_lookup.contains_key(&file_path_hash) {
            return Some(self.object_stream_manager.path_to_directory_lookup[&file_path_hash].clone())
        }
        
        None
    }

    pub fn load(&mut self, path: &str) -> Result<Arc<RwLock<igObjectDirectory>>, String> {
        let file_context = &self.file_context;
        let registry = &self.registry;
        let metadata_manager = &mut self.ark_core.metadata_manager;
        let ext_ref_system = &mut self.ig_ext_ref_system;
        let obj_handle_mgr = &mut self.ig_object_handle_manager;

        self.object_stream_manager.load(
            file_context,
            registry,
            metadata_manager,
            ext_ref_system,
            obj_handle_mgr,
            path,
        )
    }
}

pub fn get_platform_string(platform: IG_CORE_PLATFORM) -> String {
    if platform == IG_CORE_PLATFORM_WIN32 {
        return "win".to_string();
    } else if platform == IG_CORE_PLATFORM_ASPEN {
        return "aspenLow".to_string();
    } else if platform == IG_CORE_PLATFORM_ASPEN64 {
        return "aspenHigh".to_string();
    }

    match platform {
        IG_CORE_PLATFORM_DEFAULT => "unknown".to_string(),
        IG_CORE_PLATFORM_WII => "wii".to_string(),
        IG_CORE_PLATFORM_DURANGO => "durango".to_string(),
        IG_CORE_PLATFORM_XENON => "xenon".to_string(),
        IG_CORE_PLATFORM_PS3 => "ps3".to_string(),
        IG_CORE_PLATFORM_OSX => "osx".to_string(),
        IG_CORE_PLATFORM_WIN64 => "win64".to_string(),
        IG_CORE_PLATFORM_CAFE => "cafe".to_string(),
        IG_CORE_PLATFORM_RASPI => "raspi".to_string(),
        IG_CORE_PLATFORM_ANDROID => "android".to_string(),
        IG_CORE_PLATFORM_LGTV => "lgtv".to_string(),
        IG_CORE_PLATFORM_PS4 => "ps4".to_string(),
        IG_CORE_PLATFORM_WP8 => "wp8".to_string(),
        IG_CORE_PLATFORM_LINUX => "linux".to_string(),
        IG_CORE_PLATFORM_NX => "nx".to_string(),
        _ => panic!("Missing platform string for {}", platform),
    }
}
