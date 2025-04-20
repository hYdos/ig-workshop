use crate::core::fs::Endian;
use crate::core::fs::Endian::{Big, Little};
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_objects::igObjectDirectory;
use crate::core::ig_registry::igRegistry;
use crate::core::load::ig_loader::igObjectLoader;
use crate::core::memory::igMemoryPool;
use crate::core::meta::ig_metadata_manager::{igMetadataManager, igObjectMeta};
use crate::util::byteorder_fixes::{read_string, read_u32};
use log::{debug, error, info};
use std::io::{Cursor, Seek, SeekFrom};
use std::str::FromStr;
use std::sync::Arc;

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
        ig_metadata_manager: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
        file_path: &str,
    ) {
        igIGZLoader::read(
            ig_file_context,
            ig_registry,
            ig_metadata_manager,
            dir,
            file_path,
            true,
        );
    }
}

pub struct igIGZLoader {}

/// Internal type to store while jumping around to other methods
struct LoaderContext {
    /// igz version
    version: u32,
    /// unsure on what this is for
    meta_object_version: u32,
    /// platform the igz targets
    platform: IG_CORE_PLATFORM,
    /// The amount of sections present in an igz
    section_count: u32,
    /// amount of fixups present
    fixup_count: u32,
    /// Set containing all loaded memory pools. Its size is hardcoded to be 0x1F
    loaded_pools: [igMemoryPool; 0x1F],
    /// List of pointers pointing to ???, Its size is hardcoded to be 0x1F
    loaded_pointers: [u32; 0x1F],
    /// Offset where fixup's are present
    fixup_offset: u32,
    /// A list of all igObject instances present inside the igz
    vtbl_list: Vec<Arc<igObjectMeta>>,
}

impl igIGZLoader {
    fn read(
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        imm: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
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
                    error!(
                        "Failed to load igz {}. Magic value was wrong. Got: {}",
                        file_path, magic
                    );
                    panic!("Alchemy Error! Check the logs.")
                }
            }

            let version = read_u32(&mut handle, &fd.endianness).unwrap();
            let meta_object_version = read_u32(&mut handle, &fd.endianness).unwrap();
            let platform = imm.get_enum::<IG_CORE_PLATFORM>(
                read_u32(&mut handle, &fd.endianness).unwrap() as usize,
            );
            let fixup_count = read_u32(&mut handle, &fd.endianness).unwrap();

            let mut shared_state = LoaderContext {
                version,
                meta_object_version,
                platform,
                section_count: 0,
                fixup_count,
                loaded_pools: Default::default(),
                loaded_pointers: Default::default(),
                fixup_offset: 0,
                vtbl_list: vec![],
            };

            igIGZLoader::parse_sections(&mut handle, &fd.endianness, &mut shared_state);
            igIGZLoader::process_fixup_sections(
                &mut handle,
                &fd.endianness,
                &mut shared_state,
                imm,
                dir,
            );
            igIGZLoader::read_objects(&mut handle, &mut shared_state);
        } else {
            error!("Failed to load igz {}. File could not be read.", file_path);
            panic!("Alchemy Error! Check the logs.")
        }
    }

    fn parse_sections(
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        shared_state: &mut LoaderContext,
    ) {
        for i in 0..0x20 {
            handle.seek(SeekFrom::Start(0x14 + 0x10 * i)).unwrap();
            let mem_pool_name_ptr = read_u32(handle, endian).unwrap();
            let offset = read_u32(handle, endian).unwrap();
            let _length = read_u32(handle, endian).unwrap();
            let _alignment = read_u32(handle, endian).unwrap();

            if offset == 0 {
                shared_state.section_count = i as u32;
                break;
            }

            handle
                .seek(SeekFrom::Start((0x224 + mem_pool_name_ptr) as u64))
                .unwrap();
            let memory_pool_name = read_string(handle).unwrap();
            if i > 0 {
                shared_state.loaded_pools[(i - 1) as usize] =
                    igMemoryPool::from_str(&memory_pool_name).expect("Invalid memory pool name");
                shared_state.loaded_pointers[(i - 1) as usize] = offset;
            } else {
                shared_state.fixup_offset = offset;
            }
        }
    }

    fn process_fixup_sections(
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        shared_state: &mut LoaderContext,
        imm: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
    ) {
        let mut bytes_processed = 0;

        for i in 0..shared_state.fixup_count {
            handle
                .seek(SeekFrom::Start(
                    (shared_state.fixup_offset + bytes_processed) as u64,
                ))
                .unwrap();
            let magic = read_u32(handle, endian).unwrap();
            let count = read_u32(handle, endian).unwrap();
            let length = read_u32(handle, endian).unwrap();
            let start = read_u32(handle, endian).unwrap();
            handle
                .seek(SeekFrom::Start(
                    (shared_state.fixup_offset + bytes_processed + start) as u64,
                ))
                .unwrap();

            debug!(
                "Processing {}",
                String::from_utf8_lossy(&magic.to_le_bytes())
            );
            match &magic.to_le_bytes() {
                b"TMET" => {
                    // (T)ouch up (MET)adata
                    for _j in 0..count {
                        let base_pos = handle.position();
                        let vtbl_name = read_string(handle).unwrap();
                        shared_state.vtbl_list.push(imm.get_or_create_meta(&vtbl_name).unwrap());
                        debug!("IGZ contains igObject of type {}", vtbl_name);
                        

                        let bits: i32 = if shared_state.version > 7 { 2 } else { 1 };
                        handle
                            .seek(SeekFrom::Start(
                                base_pos
                                    + bits as u64
                                    + ((handle.position() - base_pos - 1)
                                        & ((-bits) as u32) as u64),
                            ))
                            .unwrap();
                    }
                }
                b"ONAM" => {
                    // (O)bjects (NAM)ed
                    dir.use_name_list = true;
                    // dir.name_list =
                }

                _ => {
                    error!(
                        "Unknown fixup present. Magic value is {}",
                        String::from_utf8_lossy(&magic.to_le_bytes())
                    )
                }
            }

            bytes_processed += length;
        }
    }

    fn read_objects(handle: &mut Cursor<Vec<u8>>, shared_state: &mut LoaderContext) {}
}
