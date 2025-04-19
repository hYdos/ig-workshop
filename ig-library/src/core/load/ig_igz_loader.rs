use std::str::FromStr;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_objects::igObjectDirectory;
use crate::core::ig_registry::igRegistry;
use crate::core::load::ig_loader::igObjectLoader;
use log::{error, info};
use crate::core::fs::Endian::{Big, Little};
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::util::byteorder_fixes::read_u32;

const IGZ_LITTLE_ENDIAN_MAGIC: u32 = u32::from_be_bytes([b'I', b'G', b'Z', 0x01]);
const IGZ_BIG_ENDIAN_MAGIC: u32 = u32::from_le_bytes([b'I', b'G', b'Z', 0x01]);

pub struct igIGZObjectLoader;

impl igObjectLoader for igIGZObjectLoader {
    fn can_read(&self, file_name: &str) -> bool {
        file_name.ends_with(".igz")
    }

    fn get_name(&self) -> &'static str {
        "Alchemy Platform"
    }

    fn get_type(&self) -> &'static str {
        "Alchemy"
    }

    fn read_file(
        &self,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_metadata_manager: &igMetadataManager,
        dir: &igObjectDirectory,
        file_path: &str,
    ) {
        igIGZLoader::read(ig_file_context, ig_registry, ig_metadata_manager, dir, file_path, true);
    }
}

pub struct igIGZLoader {}

impl igIGZLoader {
    fn read(
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_metadata_manager: &igMetadataManager,
        _dir: &igObjectDirectory,
        file_path: &str,
        _read_dependencies: bool,
    ) {
        let mut fd = ig_file_context.open(ig_registry, file_path, 0);
        if let Some(mut handle) = fd._handle {
            let magic = read_u32(&mut handle, &Little).unwrap();
            match magic {
                IGZ_BIG_ENDIAN_MAGIC => fd.endianness = Big,
                IGZ_LITTLE_ENDIAN_MAGIC => fd.endianness = Little,
                _ => {
                    error!("Failed to load igz {}. Magic value was wrong. Got: {}", file_path, magic);
                    panic!("Alchemy Error! Check the logs.")
                }
            }
            
            let _version = read_u32(&mut handle, &fd.endianness).unwrap();
            let _meta_object_version = read_u32(&mut handle, &fd.endianness).unwrap();
            let _platform = ig_metadata_manager.get_enum(read_u32(&mut handle, &fd.endianness).unwrap(), IG_CORE_PLATFORM::from_str);
            
            info!("test")
            
        } else {
            error!("Failed to load igz {}. File could not be read.", file_path);
            panic!("Alchemy Error! Check the logs.")
        }
    }
}

impl igIGZLoader {}
