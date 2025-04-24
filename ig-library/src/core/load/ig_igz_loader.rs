use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_external_ref::igExternalReferenceSystem;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_fs::Endian;
use crate::core::ig_fs::Endian::{Big, Little};
use crate::core::ig_handle::{igHandle, igHandleName};
use crate::core::ig_lists::igObjectList;
use crate::core::ig_memory::igMemoryPool;
use crate::core::ig_objects::{igObject, igObjectDirectory, igObjectStreamManager};
use crate::core::ig_registry::igRegistry;
use crate::core::load::ig_loader::igObjectLoader;
use crate::core::meta::ig_metadata_manager::igMetaObject;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::util::byteorder_fixes::{
    read_ptr, read_string, read_struct_array_u8, read_u32, read_u64,
};
use crate::util::ig_name::igName;
use log::{debug, error, warn};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom};
use std::str::FromStr;
use std::sync::{Arc, RwLock};

const IGZ_LITTLE_ENDIAN_MAGIC: u32 = u32::from_be_bytes([b'I', b'G', b'Z', 0x01]);
const IGZ_BIG_ENDIAN_MAGIC: u32 = u32::from_le_bytes([b'I', b'G', b'Z', 0x01]);

pub struct igIGZObjectLoader;

#[derive(Debug)]
enum Fixup {
    T_METADATA,
    T_DEPENDENCIES,
    T_STRING_LIST,
    EXTERNAL_DEPENDENCIES_BY_ID,
    EXTERNAL_DEPENDENCIES_BY_NAME,
    THUMBNAIL,
    RUNTIME_V_TABLES,
    RUNTIME_OBJECT_LISTS,
    RUNTIME_OFFSETS,
    RUNTIME_POOL_IDS,
    RUNTIME_STRING_TABLES,
    RUNTIME_STRING_REFERENCES,
    RUNTIME_MEMORY_HANDLES,
    RUNTIME_EXTERNALS,
    RUNTIME_NAMED_EXTERNALS,
    RUNTIME_HANDLES,
    OPTION_NAMED_LIST,
}

impl Fixup {
    fn fix(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        imm: &mut igMetadataManager,
        length: u32,
        start: u32,
        count: u32,
        dir: &mut igObjectDirectory,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        ctx: &mut LoaderContext,
    ) {
        match self {
            Fixup::T_DEPENDENCIES => {
                if ctx.read_dependencies {
                    for _i in 0..count {
                        let name = read_string(handle).unwrap();
                        let path = read_string(handle).unwrap();
                        if path.starts_with("<build>") {
                            // Unsure on why cauldron does this
                            continue;
                        }
                        let name = igName::new(name);
                        if let Ok(dependency) = ig_object_stream_manager.load_with_namespace(
                            ig_file_context,
                            ig_registry,
                            imm,
                            ig_ext_ref_system,
                            path.clone(),
                            name,
                        ) {
                            dir.dependencies.push(dependency)
                        } else {
                            error!("Failed to find dependency {}", path);
                        }
                    }
                }
            }
            Fixup::T_METADATA => {
                for _i in 0..count {
                    let base_pos = handle.position();
                    let vtbl_name = read_string(handle).unwrap();
                    ctx.vtbl_list
                        .push(imm.get_or_create_meta(&vtbl_name).unwrap());
                    debug!("IGZ contains igObject of type {}", vtbl_name);

                    let bits: i32 = if ctx.version > 7 { 2 } else { 1 };
                    handle
                        .seek(SeekFrom::Start(
                            base_pos
                                + bits as u64
                                + ((handle.position() - base_pos - 1) & ((-bits) as u32) as u64),
                        ))
                        .unwrap();
                }
            }

            Fixup::T_STRING_LIST => {
                for _i in 0..count {
                    let base_pos = handle.position();
                    let data = read_string(handle).unwrap();
                    ctx.string_list.push(data);

                    let bits: i32 = if ctx.version > 7 { 2 } else { 1 };
                    handle
                        .seek(SeekFrom::Start(
                            base_pos
                                + bits as u64
                                + ((handle.position() - base_pos - 1) & ((-bits) as u32) as u64),
                        ))
                        .unwrap();
                }
            }
            Fixup::EXTERNAL_DEPENDENCIES_BY_ID => {
                for _i in 0..count {
                    let dependency_name = igHandleName::new(
                        igName::from_hash(read_u32(handle, endian).unwrap()), // name
                        igName::from_hash(read_u32(handle, endian).unwrap()), // namespace
                    );

                    let mut obj = None;
                    if let Some(list) = ig_object_stream_manager
                        .name_to_directory_lookup
                        .get(&dependency_name.namespace.hash)
                    {
                        for dependant_dir in list.iter() {
                            if let Ok(dependent_dir) = dependant_dir.try_read() {
                                if dependent_dir.use_name_list {
                                    for i in 0..dependent_dir.name_list.len() {
                                        let name = &dependent_dir.name_list.query()[i];
                                        if name.hash == dependency_name.namespace.hash {
                                            obj =
                                                Some(dependent_dir.object_list.query()[i].clone());
                                            break;
                                        }
                                    }

                                    if obj.is_some() {
                                        break;
                                    }
                                }
                            } else {
                                error!("Failed to get read lock on igObjectDirectory");
                                panic!("Alchemy Error! Check the logs.")
                            }
                        }
                    } else {
                        warn!("EXIT Fixup load failed: Failed to find namespace {:#01}, referenced in {}", dependency_name.namespace.hash, dir.path);
                    }
                }
            }
            Fixup::EXTERNAL_DEPENDENCIES_BY_NAME => {
                for _i in 0..count {
                    let raw_handle = read_u64(handle, endian).unwrap();
                    let ns_str_index = (raw_handle >> 32) as u32;
                    let name_str_index = raw_handle as u32;
                    let dependency_handle_name = igHandleName::new(
                        igName::new(ctx.string_list[name_str_index as usize].clone()),
                        igName::new(ctx.string_list[ns_str_index as usize].clone()),
                    );

                    let mut obj = None;
                    if let Some(dependant_dir) = dir.dependencies.iter().find(|dependency| {
                        let guard = dependency.read().unwrap();
                        guard.name.hash == dependency_handle_name.namespace.hash
                    }) {
                        if let Ok(dependent_dir) = dependant_dir.try_read() {
                            if dependent_dir.use_name_list {
                                for i in 0..dependent_dir.name_list.len() {
                                    let name = &dependent_dir.name_list.query()[i];
                                    if name.hash == dependency_handle_name.namespace.hash {
                                        obj = Some(dependent_dir.object_list.query()[i].clone());
                                        break;
                                    }
                                }

                                if obj.is_some() {
                                    break;
                                }
                            }
                        } else {
                            error!("Failed to get read lock on igObjectDirectory");
                            panic!("Alchemy Error! Check the logs.")
                        }
                    }

                    let dependency_handle = igHandle::from_handle_name(&dependency_handle_name);
                    if (ns_str_index & 0x80000000) != 0 {
                        ctx.named_handle_list.push(dependency_handle.clone());
                    } else {
                        let mut reference = ig_ext_ref_system
                            .global_set
                            .resolve_reference(&dependency_handle_name);
                        if reference.is_none() {
                            reference = Some(dependency_handle.read().unwrap().get_object_alias())
                        }
                        ctx.named_external_list.push(reference.unwrap());
                    }
                }
            }
            Fixup::THUMBNAIL => {
                for _i in 0..count {
                    let size = read_ptr(handle, &ctx.platform, &endian).unwrap();
                    let raw = read_ptr(handle, &ctx.platform, &endian).unwrap();
                    ctx.thumbnails.push((size, raw))
                }
            }
            Fixup::RUNTIME_V_TABLES => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.vtables = unpack_compressed_ints(ctx, &vec, count, false);
                instantiate_and_append_objects(ctx, handle, endian);
            }
            Fixup::RUNTIME_OBJECT_LISTS => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.vtables = unpack_compressed_ints(ctx, &vec, count, false);
                dir.object_list =
                    ctx.offset_object_list[&ctx.runtime_fields.object_lists[0]].clone()
            }
            Fixup::RUNTIME_OFFSETS => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields._offsets = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_POOL_IDS => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields._pool_ids = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_STRING_TABLES => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.string_tables = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_STRING_REFERENCES => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.string_references =
                    unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_MEMORY_HANDLES => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.memory_handles = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_EXTERNALS => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.externals = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_NAMED_EXTERNALS => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.named_externals = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::RUNTIME_HANDLES => {
                let vec = read_struct_array_u8(handle, endian, (length - start) as usize).unwrap();
                ctx.runtime_fields.handles = unpack_compressed_ints(ctx, &vec, count, true);
            }
            Fixup::OPTION_NAMED_LIST => {
                dir.use_name_list = true;
                dir.name_list = todo!("");
            }
        }
    }
}

fn instantiate_and_append_objects(
    ctx: &mut LoaderContext,
    handle: &mut Cursor<Vec<u8>>,
    endian: &Endian,
) {
    for vtable in &ctx.runtime_fields.vtables {
        // TODO: cast to a igObjectList (Vec<igObject>, a implemented serialization of __internalObjectBase
        ctx.offset_object_list.insert(
            *vtable,
            instantiate_object(ctx, handle, endian, vtable)
                .read()
                .unwrap()
                .as_any()
                .downcast_ref::<igObjectList>()
                .unwrap()
                .to_owned(),
        );
    }
}

fn instantiate_object(
    ctx: &LoaderContext,
    handle: &mut Cursor<Vec<u8>>,
    endian: &Endian,
    offset: &u64,
) -> igObject {
    handle
        .seek(SeekFrom::Start(deserialize_offset(ctx, *offset)))
        .unwrap();
    let offset = read_ptr(handle, &ctx.platform, endian).unwrap();
    ctx.vtbl_list[offset as usize]
        .instantiate(get_mem_pool_from_serialized_offset(ctx, offset), false)
        .unwrap()
}

fn get_mem_pool_from_serialized_offset(ctx: &LoaderContext, offset: u64) -> igMemoryPool {
    if ctx.version <= 0x06 {
        ctx.loaded_pools[(offset >> 0x18) as usize]
    } else {
        ctx.loaded_pools[(offset >> 0x1B) as usize]
    }
}

fn unpack_compressed_ints(
    ctx: &mut LoaderContext,
    bytes: &[u8],
    count: u32,
    deserialize: bool,
) -> Vec<u64> {
    let mut output = Vec::with_capacity(count as usize);
    let mut prev_int: u32 = 0;
    let mut shift_move_or_mask = false;
    let mut idx: usize = 0;

    for _ in 0..count {
        let mut current = if !shift_move_or_mask {
            let b = bytes[idx];
            shift_move_or_mask = true;
            (b & 0xF) as u32
        } else {
            let b = bytes[idx];
            shift_move_or_mask = false;
            idx += 1;
            (b >> 4) as u32
        };

        let mut unpacked = current & 0x7;
        let mut shift_amount = 3;

        while (current & 0x8) != 0 {
            current = if !shift_move_or_mask {
                let b = bytes[idx];
                shift_move_or_mask = true;
                (b & 0xF) as u32
            } else {
                let b = bytes[idx];
                shift_move_or_mask = false;
                idx += 1;
                (b >> 4) as u32
            };
            unpacked |= (current & 0x7) << (shift_amount & 0x1F);
            shift_amount += 3;
        }

        // delta‑and‑scale, plus version‑dependent bias
        prev_int = prev_int
            .wrapping_add(unpacked * 4)
            .wrapping_add(if ctx.version < 9 { 4 } else { 0 });

        let final_val = if deserialize {
            deserialize_offset(ctx, prev_int as u64)
        } else {
            prev_int as u64
        };

        output.push(final_val);
    }

    output
}

fn deserialize_offset(ctx: &LoaderContext, offset: u64) -> u64 {
    if ctx.version <= 6 {
        ctx.loaded_pointers[((offset >> 0x18) + (offset & 0x00FFFFFF)) as usize] as u64
    } else {
        ctx.loaded_pointers[((offset >> 0x1B) + (offset & 0x00FFFFFF)) as usize] as u64
    }
}

/// TryFrom<u32>'s implementation here has a conversion table for names of fixups from any igz versioned 7 or above.
impl TryFrom<u32> for Fixup {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match &value.to_le_bytes() {
            b"TDEP" => Ok(Fixup::T_DEPENDENCIES),
            b"TMET" => Ok(Fixup::T_METADATA),
            b"TSTR" => Ok(Fixup::T_STRING_LIST),
            b"EXID" => Ok(Fixup::EXTERNAL_DEPENDENCIES_BY_ID),
            b"EXNM" => Ok(Fixup::EXTERNAL_DEPENDENCIES_BY_NAME),
            b"TMHN" => Ok(Fixup::THUMBNAIL),
            b"RVTB" => Ok(Fixup::RUNTIME_V_TABLES),
            b"ROOT" => Ok(Fixup::RUNTIME_OBJECT_LISTS),
            b"ROFS" => Ok(Fixup::RUNTIME_OFFSETS),
            b"RPID" => Ok(Fixup::RUNTIME_POOL_IDS),
            b"RSTT" => Ok(Fixup::RUNTIME_STRING_TABLES),
            b"RSTR" => Ok(Fixup::RUNTIME_STRING_REFERENCES),
            b"RMHN" => Ok(Fixup::RUNTIME_MEMORY_HANDLES),
            b"REXT" => Ok(Fixup::RUNTIME_EXTERNALS),
            b"RNEX" => Ok(Fixup::RUNTIME_NAMED_EXTERNALS),
            b"RHND" => Ok(Fixup::RUNTIME_HANDLES),
            b"ONAM" => Ok(Fixup::OPTION_NAMED_LIST),
            _ => Err(()),
        }
    }
}

/// TryFrom<u8>'s implementation here has a conversion table for id's of fixups from any igz versioned 6 or below.
impl TryFrom<u8> for Fixup {
    type Error = ();

    fn try_from(_value: u8) -> Result<Self, Self::Error> {
        todo!("SSA Wii (version 6?) fixup's are not implemented")
    }
}

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
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        ig_metadata_manager: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
        file_path: &str,
    ) {
        igIGZLoader::read(
            ig_file_context,
            ig_registry,
            ig_object_stream_manager,
            ig_ext_ref_system,
            ig_metadata_manager,
            dir,
            file_path,
            true,
        );
    }
}

pub struct igIGZLoader {}

/// See comment in [LoaderContext]
struct RuntimeFields {
    vtables: Vec<u64>,
    object_lists: Vec<u64>,
    _offsets: Vec<u64>,
    _pool_ids: Vec<u64>,
    string_tables: Vec<u64>,
    string_references: Vec<u64>,
    memory_handles: Vec<u64>,
    externals: Vec<u64>,
    named_externals: Vec<u64>,
    handles: Vec<u64>,
}

impl RuntimeFields {
    fn new() -> RuntimeFields {
        RuntimeFields {
            vtables: vec![],
            object_lists: vec![],
            _offsets: vec![],
            _pool_ids: vec![],
            string_tables: vec![],
            string_references: vec![],
            memory_handles: vec![],
            externals: vec![],
            named_externals: vec![],
            handles: vec![],
        }
    }
}

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
    vtbl_list: Vec<Arc<igMetaObject>>,
    /// A list of all strings present inside the igz
    string_list: Vec<String>,
    /// A list of all external ig object dependencies needed
    named_external_list: Vec<igObject>,
    /// A list of all handles used from dependencies
    named_handle_list: Vec<Arc<RwLock<igHandle>>>,
    /// Setting decides if the dependency fixup will try load dependencies
    read_dependencies: bool,
    /// A list of all thumbnails present in the igz.
    thumbnails: Vec<(u64, u64)>,
    /// All runtime lists stored from fixups. Used for various parts of the runtime
    runtime_fields: RuntimeFields,
    /// TODO: comment
    offset_object_list: HashMap<u64, igObjectList>,
}

impl igIGZLoader {
    fn read(
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        imm: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
        file_path: &str,
        read_dependencies: bool,
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
                string_list: vec![],
                named_external_list: vec![],
                named_handle_list: vec![],
                read_dependencies,
                thumbnails: vec![],
                runtime_fields: RuntimeFields::new(),
                offset_object_list: HashMap::new(),
            };

            igIGZLoader::parse_sections(&mut handle, &fd.endianness, &mut shared_state);
            igIGZLoader::process_fixup_sections(
                &mut handle,
                &fd.endianness,
                &mut shared_state,
                ig_file_context,
                ig_registry,
                ig_object_stream_manager,
                ig_ext_ref_system,
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
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        imm: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
    ) {
        let mut bytes_processed = 0;

        for _i in 0..shared_state.fixup_count {
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

            let fixup = Fixup::try_from(magic);
            if let Ok(fixup) = fixup {
                debug!(
                    "Processing {}",
                    String::from_utf8_lossy(&magic.to_le_bytes())
                );
                fixup.fix(
                    handle,
                    endian,
                    imm,
                    length,
                    start,
                    count,
                    dir,
                    ig_file_context,
                    ig_registry,
                    ig_object_stream_manager,
                    ig_ext_ref_system,
                    shared_state,
                );
            } else {
                debug!(
                    "No fixup exists for the magic value {}",
                    String::from_utf8_lossy(&magic.to_le_bytes())
                )
            }

            bytes_processed += length;
        }
    }

    fn read_objects(_handle: &mut Cursor<Vec<u8>>, _shared_state: &mut LoaderContext) {}
}
