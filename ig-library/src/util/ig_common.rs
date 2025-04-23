use crate::client::client::CClient;
use crate::core::ig_external_ref::igExternalReferenceSystem;
use crate::core::ig_ark_core::igArkCore;
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_core_platform::IG_CORE_PLATFORM::*;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_objects::igObjectStreamManager;
use crate::core::ig_registry::igRegistry;

/// After early initialization, this type becomes available to make getting state a less painful task
pub struct igAlchemy {
    pub ark_core: igArkCore,
    pub file_context: igFileContext,
    pub registry: igRegistry,
    pub object_stream_manager: igObjectStreamManager,
    pub ig_ext_ref_system: igExternalReferenceSystem,
    pub client: CClient,
}

impl igAlchemy {
    pub fn new(ig_file_context: igFileContext, ig_registry: igRegistry, ig_ark_core: igArkCore) -> igAlchemy {
        igAlchemy {
            ark_core: ig_ark_core,
            file_context: ig_file_context,
            object_stream_manager: igObjectStreamManager::new(),
            ig_ext_ref_system: igExternalReferenceSystem::new(),
            client: CClient::init(&ig_registry),
            registry: ig_registry,
        }
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
        _ => panic!("Missing platform string for {}", platform),
    }
}
