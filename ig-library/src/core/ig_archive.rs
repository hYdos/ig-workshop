use crate::core::ig_file_context::WorkStatus::{
    kStatusComplete, kStatusGeneralError, kStatusInvalidPath, kStatusUnsupported,
};
use crate::core::ig_file_context::{igFileContext, igFileWorkItem, WorkItemBuffer};
use crate::core::ig_fs::{igFileWorkItemProcessor, igStorageDevice, Endian};
use crate::core::ig_registry::{igRegistry, BuildTool};
use crate::util::byteorder_fixes::{
    read_string, read_struct_array_u16, read_struct_array_u32, read_struct_array_u8,
    read_struct_array_u8_ref, read_u32, read_u64,
};
use crate::util::ig_hash;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::DeflateDecoder;
use log::debug;
use lzma_rust2::LZMAReader;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

/// Represents an archive file
pub struct igArchive {
    next_processor: Option<Arc<RwLock<dyn igFileWorkItemProcessor>>>,
    pub _path: String,
    pub _name: String,
    pub _load_name_table: bool,
    pub _sequential_read: bool,
    pub _loading_for_incremental_update: bool,
    pub _enable_cache: bool,
    pub _override: bool,
    pub _open: bool,
    pub _configured: bool,
    pub _needs_endian_swap: bool,
    pub _archive_header: Header,
    pub _files: Vec<FileInfo>,
    pub _native_media: String,
    pub _native_path: String,
    pub _native_app_path: String,
}

/// Deletes a file present in an archive
fn delete(_path: &str) -> Result<(), ()> {
    todo!("implement delete file in igArchive")
}

impl igArchive {
    pub fn hash_file_path(&self, file_path: &str) -> u32 {
        let mut path_copy = file_path.to_string();

        // kCaseInsensitiveHash
        if (self._archive_header._flags & 1u32) != 0 {
            path_copy = path_copy.replace("\\", "/");
            path_copy = path_copy.to_lowercase();
        }

        // kHashNameAndExtensionOnly
        if (self._archive_header._flags & 2u32) != 0 {
            path_copy = Path::new(&path_copy)
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("")
                .to_string();
        }

        path_copy = path_copy.trim_start_matches(['/', '\\']).to_string();
        ig_hash::hash(&path_copy)
    }

    /// Reverse engineered by DTZxPorter. It will Search the list of files for a given hash and will return the index of the file info
    pub fn hash_search(
        file_info: &[FileInfo],
        hash_search_divider: u32,
        hash_search_slop: u32,
        file_hash: u32,
    ) -> Option<usize> {
        let mut file_count = file_info.len() as u32;
        let mut file_hash_divided = file_hash / hash_search_divider; // most likely an optimization to make searching easier when fewer collisions can happen.

        let mut search_at = 0;
        if hash_search_slop < file_hash_divided {
            search_at = file_hash_divided - hash_search_slop;
        }

        file_hash_divided += hash_search_slop + 1;
        if file_hash_divided < file_count {
            file_count = file_hash_divided;
        }

        let mut index = search_at;
        search_at = file_count - index;
        let mut i = search_at;
        while 0 < i {
            i = search_at / 2;
            if file_info[(index + i) as usize]._hash < file_hash {
                index += i + 1;
                i = search_at - 1 - i;
            }
            search_at = i;
        }

        if index < file_info.len() as u32 && file_info[index as usize]._hash == file_hash {
            return Some(index as usize);
        }

        None // no file was found :(
    }

    /// Will check if the archive contains a file based on the path provided
    fn has_file(&self, path: &str) -> bool {
        self.has_hash(self.hash_file_path(path))
    }

    /// Similar to [has_file], but will use a hash instead of a file path.
    fn has_hash(&self, _hash: u32) -> bool {
        Self::hash_search(
            &self._files,
            self._archive_header._hash_search_divider,
            self._archive_header._hash_search_slop,
            _hash,
        )
        .is_some()
    }

    fn decompress_as_handle(&self, file_info: &FileInfo) -> Cursor<Vec<u8>> {
        Cursor::new(self.decompress(file_info))
    }

    fn decompress(&self, file_info: &FileInfo) -> Vec<u8> {
        let mut dst = Vec::<u8>::new();
        if file_info._block_index == 0xFFFFFFFF {
            dst.write_all(&file_info._compressed_data).unwrap();
            return dst;
        }
        let blocks = file_info._blocks.clone().unwrap();
        let compression_type = CompressionType::from_index(file_info._block_index);
        for i in 0..blocks.len() {
            let decompressed_size = if file_info._length < ((i + 1) * 0x8000) as u32 {
                file_info._length & 0x7FFF
            } else {
                0x8000
            };

            // let should_decompress = blocks[i] & 0x80000000u32 != 0; // unused
            let mut offset =
                ((blocks[i] & 0x7FFFFFFF) * self._archive_header._sector_size) as usize;
            if blocks[i] & 0x80000000u32 == 0 {
                let compressed_data =
                    &file_info._compressed_data[offset..(offset + decompressed_size as usize)];
                dst.write_all(compressed_data).unwrap();
                continue;
            }

            let mut cursor = Cursor::new(&file_info._compressed_data);
            cursor.seek(SeekFrom::Start(offset as u64)).unwrap();
            let compressed_size = cursor.read_u16::<LittleEndian>().unwrap(); // C# Implementation does not state what endian igCauldron is reading this in.
            drop(cursor);
            offset += 2;

            match compression_type {
                CompressionType::kZlib => {
                    let slice =
                        &file_info._compressed_data[offset..offset + compressed_size as usize];
                    let mut decoder = DeflateDecoder::new(slice);
                    decoder.read_to_end(&mut dst).unwrap();
                }
                CompressionType::kLzma => {
                    let lzma_properties = &file_info._compressed_data[offset..offset + 5];
                    let slice = &file_info._compressed_data
                        [offset + 5..(5 + offset + compressed_size as usize)];

                    let first = lzma_properties[0] as usize;
                    let lc = first % 9;
                    let num = first / 9;
                    let lp = num % 5;
                    let pb = num / 5;

                    // reconstruct little‑endian dictionary size from bytes 1..5
                    let mut dictionary_size: u32 = 0;
                    for i in 0..4 {
                        dictionary_size = dictionary_size
                            .wrapping_add((lzma_properties[1 + i] as u32) << (i * 8));
                    }

                    let mut buf = [0; 1024];
                    let mut reader = LZMAReader::new(
                        Cursor::new(slice),
                        decompressed_size as u64,
                        lc as u32,
                        lp as u32,
                        pb as u32,
                        dictionary_size,
                        None,
                    )
                    .unwrap();

                    loop {
                        let n = reader.read(&mut buf).unwrap();
                        if n == 0 {
                            break;
                        }
                        dst.extend_from_slice(&buf[..n]);
                    }
                }
                CompressionType::kLz4 => {
                    let slice =
                        &file_info._compressed_data[offset..(offset + compressed_size as usize)];
                    dst.extend(
                        lz4::block::decompress(slice, Some(decompressed_size as i32)).unwrap(),
                    );
                }
                _ => panic!("Unsupported compression type {:?}", compression_type),
            }
        }

        dst
    }

    /// Opens an archive
    /// file_path is the path of the archive
    pub fn open(
        file_context: &igFileContext,
        ig_registry: &igRegistry,
        file_path: &str,
    ) -> Result<igArchive, String> {
        let mut file_descriptor = file_context.open(ig_registry, file_path, 0);
        let _path = file_descriptor._path;
        let mut header = Header {
            endian: Endian::Little,
            _magic_number: 0,
            _version: 0,
            _toc_size: 0,
            _num_files: 0,
            _sector_size: 0,
            _hash_search_divider: 0,
            _hash_search_slop: 0,
            _num_large_file_blocks: 0,
            _num_medium_file_blocks: 0,
            _num_small_file_blocks: 0,
            _name_table_offset: 0,
            _name_table_size: 0,
            _flags: 0,
        };

        if let Some(mut cursor) = file_descriptor._handle {
            cursor.seek(SeekFrom::Start(0)).unwrap();
            header._magic_number = read_u32(&mut cursor, &Endian::Little).unwrap();

            if header._magic_number == u32::from_be_bytes(*b"IGA\x1A") {
                header.endian = Endian::Big;
            } else if header._magic_number != u32::from_le_bytes(*b"IGA\x1A") {
                return Err(format!("{} is not a valid igArchive.", _path));
            }

            header._version = read_u32(&mut cursor, &header.endian).unwrap();
            match header._version {
                // Crash Team Racing: Nitro Fueled, Crash NST, Trap Team, Superchargers, Imaginators
                0x0A..=0x0D => {
                    header._toc_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_files = read_u32(&mut cursor, &header.endian).unwrap();
                    header._sector_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._hash_search_divider = read_u32(&mut cursor, &header.endian).unwrap();
                    header._hash_search_slop = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_large_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_medium_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_small_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._name_table_offset = read_u64(&mut cursor, &header.endian).unwrap();
                    header._name_table_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._flags = read_u32(&mut cursor, &header.endian).unwrap();
                }
                // TODO: lost islands/ssf (version 0x0A)
                // SSA(WiiU), SG
                0x08 => {
                    header._toc_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_files = read_u32(&mut cursor, &header.endian).unwrap();
                    header._sector_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._hash_search_divider = read_u32(&mut cursor, &header.endian).unwrap();
                    header._hash_search_slop = read_u32(&mut cursor, &header.endian).unwrap();
                    header._name_table_offset = read_u32(&mut cursor, &header.endian).unwrap() as u64;
                    header._name_table_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_large_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_medium_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_small_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._flags = read_u32(&mut cursor, &header.endian).unwrap();
                }
                0x04 => {
                    header._toc_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_files = read_u32(&mut cursor, &header.endian).unwrap();
                    header._sector_size = 0x0800;
                    header._hash_search_divider = read_u32(&mut cursor, &header.endian).unwrap();
                    header._hash_search_slop = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_large_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_medium_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._num_small_file_blocks = read_u32(&mut cursor, &header.endian).unwrap();
                    header._name_table_offset =
                        read_u32(&mut cursor, &header.endian).unwrap() as u64;
                    header._name_table_size = read_u32(&mut cursor, &header.endian).unwrap();
                    header._flags = read_u32(&mut cursor, &header.endian).unwrap();
                    return Err(format!(
                        "igArchive version {} is not implemented.",
                        header._version
                    ));
                }
                _ => {
                    return Err(format!(
                        "igArchive version {} is not implemented.",
                        header._version
                    ))
                }
            }

            // File entries are stored in three sections: one section stores the hash, the second gets offset and other general info, and the last has a second set of info relating to names
            let mut _files: Vec<FileInfo> = Vec::with_capacity(header._num_files as usize);
            for _i in 0..header._num_files {
                _files.push(FileInfo {
                    _offset: 0,
                    _ordinal: 0,
                    _length: 0,
                    _block_index: 0,
                    _name: "".to_string(),
                    _logical_name: "".to_string(),
                    _modification_time: 0,
                    _blocks: None,
                    _compressed_data: vec![],
                    _hash: read_u32(&mut cursor, &header.endian).unwrap(),
                })
            }

            for i in 0..header._num_files {
                let file = &mut _files[i as usize];

                match header._version {
                    0x0D => {
                        // technically the offset is 5 bytes and the ordinal is 3
                        let tmp = read_u64(&mut cursor, &header.endian).unwrap(); // Read all 8 bytes together at once
                        file._ordinal = (tmp >> 40) as u32;
                        file._offset = (tmp & 0xFFFFFFFF) as u32; // FIXME: this looks like its reading 4 bytes, not 5...
                        file._length = read_u32(&mut cursor, &header.endian).unwrap();
                        file._block_index = read_u32(&mut cursor, &header.endian).unwrap();
                    },
                    0x0B => {
                        // technically the offset is 5 bytes and the ordinal is 3
                        let tmp = read_u64(&mut cursor, &header.endian).unwrap(); // Read all 8 bytes together at once
                        file._ordinal = (tmp >> 40) as u32;
                        file._offset = (tmp & 0xFFFFFFFF) as u32; // FIXME: this looks like its reading 4 bytes, not 5...
                        file._length = read_u32(&mut cursor, &header.endian).unwrap();
                        file._block_index = read_u32(&mut cursor, &header.endian).unwrap();
                    },
                    0x08 => {
                        file._offset = read_u32(&mut cursor, &header.endian).unwrap();
                        file._length = read_u32(&mut cursor, &header.endian).unwrap();
                        file._block_index = read_u32(&mut cursor, &header.endian).unwrap();
                        // giants doesn't store the ordinal of the file?
                    }
                    _ => todo!("Unsupported IGA version"),
                }
            }

            let name_tbl_offset = header._name_table_offset;

            for i in 0..header._num_files {
                let file = &mut _files[i as usize];
                // pointer to a pointer to the name information
                cursor
                    .seek(SeekFrom::Start(name_tbl_offset + i as u64 * 0x04))
                    .unwrap();
                let inner_ptr = read_u32(&mut cursor, &header.endian).unwrap() as u64;
                cursor
                    .seek(SeekFrom::Start(name_tbl_offset + inner_ptr))
                    .unwrap();

                let name1 = read_string(&mut cursor).unwrap();
                let mut name2 = None;

                if header._version >= 0x0A {
                    name2 = Some(read_string(&mut cursor).unwrap());
                }

                if header._version >= 0x08 {
                    file._modification_time = read_u32(&mut cursor, &header.endian).unwrap();
                }
                
                // Cauldron reorganizes the names for lower versions. As far as I know, this is wrong but just in case we will handle Tfb Games the newer way because that's what we expect.
                if header._version >= 0x0B || ig_registry.build_tool == BuildTool::TfbTool {
                    file._name = name1;
                    file._logical_name = name2.unwrap_or_default();
                } else {
                    file._logical_name = name1;
                    file._name = name2.unwrap_or_default();
                }
            }

            let block_info_start = get_header_size(header._version) as u64
                + header._num_files as u64 * (0x04 + get_file_info_size(header._version)) as u64;

            cursor.seek(SeekFrom::Start(block_info_start)).unwrap();
            let large_block_tbl = read_struct_array_u32(
                &mut cursor,
                &header.endian,
                header._num_large_file_blocks as usize,
            )
            .unwrap();
            let medium_block_tbl = read_struct_array_u16(
                &mut cursor,
                &header.endian,
                header._num_medium_file_blocks as usize,
            )
            .unwrap();
            let small_block_tbl = read_struct_array_u8_ref(
                &mut cursor,
                &header.endian,
                header._num_small_file_blocks as usize,
            )
            .unwrap();

            for file in &mut _files {
                cursor.seek(SeekFrom::Start(file._offset as u64)).unwrap();
                if file._block_index == 0xFFFFFFFF {
                    file._compressed_data =
                        read_struct_array_u8(&mut cursor, &header.endian, file._length as usize)
                            .unwrap();
                    continue;
                }

                let mut sector_count = 0;
                let block_count = (file._length + 0x7FFF) >> 0xF;
                let mut fixed_blocks: Vec<u32> = Vec::with_capacity(block_count as usize);
                for _i in 0..block_count as usize {
                    fixed_blocks.push(0);
                }

                for i in 0..block_count {
                    let block_idx = ((file._block_index & 0x0FFFFFFF) + i) as usize;
                    let is_compressed;
                    let mut block;
                    if 0x7F * header._sector_size < file._length {
                        if 0x7FFF * header._sector_size < file._length {
                            block = large_block_tbl[block_idx];
                            is_compressed = (block >> 0x1F) == 1;
                            block &= 0x7FFFFFFF;
                            sector_count += (large_block_tbl[block_idx + 1] & 0x7FFFFFFF) - block;
                        } else {
                            block = medium_block_tbl[block_idx] as u32;
                            is_compressed = (block >> 0x0F) == 1;
                            block &= 0x7FFF;
                            sector_count +=
                                (medium_block_tbl[block_idx + 1] & 0x7FFF) as u32 - block;
                        }
                    } else {
                        block = small_block_tbl[block_idx] as u32;
                        is_compressed = (block >> 0x07) == 1;
                        block &= 0x7F;
                        sector_count += (small_block_tbl[block_idx + 1] & 0x7F) as u32 - block;
                    }

                    fixed_blocks[i as usize] =
                        if is_compressed { 0x80000000u32 } else { 0u32 } | block;
                }

                file._blocks = Some(fixed_blocks);
                file._compressed_data = read_struct_array_u8(
                    &mut cursor,
                    &header.endian,
                    (sector_count * header._sector_size) as usize,
                )
                .unwrap()
            }

            // Hint to the compiler to drop this as soon as possible
            file_descriptor._handle = None;

            Ok(igArchive {
                next_processor: None,
                _path,
                _name: "".to_string(),
                _load_name_table: false,
                _sequential_read: false,
                _loading_for_incremental_update: false,
                _enable_cache: false,
                _override: false,
                _open: false,
                _configured: false,
                _needs_endian_swap: false,
                _archive_header: header,
                _files,
                _native_media: "".to_string(),
                _native_path: "".to_string(),
                _native_app_path: "".to_string(),
            })
        } else {
            Err("file_descriptor._handle was not available".to_string())
        }
    }

    pub fn new() -> Self {
        igArchive {
            next_processor: None,
            _path: "".to_string(),
            _name: "".to_string(),
            _load_name_table: false,
            _sequential_read: false,
            _loading_for_incremental_update: false,
            _enable_cache: false,
            _override: false,
            _open: false,
            _configured: false,
            _needs_endian_swap: false,
            _archive_header: Header {
                endian: Endian::Little,
                _magic_number: 0,
                _version: 0,
                _toc_size: 0,
                _num_files: 0,
                _sector_size: 0,
                _hash_search_divider: 0,
                _hash_search_slop: 0,
                _num_large_file_blocks: 0,
                _num_medium_file_blocks: 0,
                _num_small_file_blocks: 0,
                _name_table_offset: 0,
                _name_table_size: 0,
                _flags: 0,
            },
            _files: vec![],
            _native_media: "".to_string(),
            _native_path: "".to_string(),
            _native_app_path: "".to_string(),
        }
    }
}

fn get_header_size(version: u32) -> u8 {
    match version {
        0x0A..=0x0D => 0x38,
        0x08 => 0x34,
        _ => panic!("IGA version {} is unsupported", version),
    }
}

fn get_file_info_size(version: u32) -> u8 {
    match version {
        0x0B => 0x10,
        0x08 => 0x0C,
        _ => panic!("IGA version {} is unsupported", version),
    }
}

impl igFileWorkItemProcessor for igArchive {
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

impl igStorageDevice for igArchive {
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
        if self.has_file(&work_item._path) {
            work_item._status = kStatusComplete;
        } else {
            work_item._status = kStatusInvalidPath;
        }
    }

    fn open(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        match work_item.ig_registry.build_tool {
            BuildTool::AlchemyLaboratory => {
                debug!(
                    "{} has hash {}",
                    work_item._path,
                    self.hash_file_path(&work_item._path)
                );
                if let Some(file_idx) = Self::hash_search(
                    &self._files,
                    self._archive_header._hash_search_divider,
                    self._archive_header._hash_search_slop,
                    self.hash_file_path(&work_item._path),
                ) {
                    let file = &self._files[file_idx];
                    work_item._file._path = work_item._path.clone();
                    work_item._file._size = file._length as u64;
                    work_item._file._position = 0;
                    work_item._file._device = Some(this.clone());
                    work_item._file._handle = Some(self.decompress_as_handle(file));
                    work_item._status = kStatusComplete;
                } else {
                    work_item._status = kStatusInvalidPath
                }
            }
            BuildTool::TfbTool => {
                let file_name = &work_item._path[(work_item._path.rfind('/').unwrap() + 1)..];
                let archive_path = &work_item._path[..work_item._path.rfind('/').unwrap()];

                if archive_path == self._path {
                    for file in &self._files {
                        if file._name == file_name {
                            work_item._file._path = work_item._path.clone();
                            work_item._file._size = file._length as u64;
                            work_item._file._position = 0;
                            work_item._file._device = Some(this.clone());
                            work_item._file._handle = Some(self.decompress_as_handle(file));
                            work_item._status = kStatusComplete;
                        }
                    }
                }
            }

            _ => panic!("Unsupported Game Tooling"),
        }
    }

    fn close(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn read(&self, _this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        work_item._status = kStatusUnsupported
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
        work_item._status = kStatusUnsupported
    }

    fn get_file_list(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        match &mut work_item._buffer {
            WorkItemBuffer::StringRefList(files) => {
                for file_info in &self._files {
                    files.push(file_info._logical_name.clone())
                }
            }
            _ => {
                work_item._status = kStatusGeneralError;
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
        if delete(&work_item._path).is_ok() {
            work_item._status = kStatusComplete
        } else {
            work_item._status = kStatusInvalidPath
        }
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

impl igFileWorkItemProcessor for Arc<igArchive> {
    fn set_next_processor(&mut self, _new_processor: Arc<RwLock<dyn igFileWorkItemProcessor>>) {
        panic!("Arc<igArchive> is not mutable.")
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

impl igStorageDevice for Arc<igArchive> {
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
        if self.has_file(&work_item._path) {
            work_item._status = kStatusComplete;
        } else {
            work_item._status = kStatusInvalidPath;
        }
    }

    fn open(&self, this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        match work_item.ig_registry.build_tool {
            BuildTool::AlchemyLaboratory => {
                debug!(
                    "{} has hash {}",
                    work_item._path,
                    self.hash_file_path(&work_item._path)
                );
                if let Some(file_idx) = igArchive::hash_search(
                    &self._files,
                    self._archive_header._hash_search_divider,
                    self._archive_header._hash_search_slop,
                    self.hash_file_path(&work_item._path),
                ) {
                    let file = &self._files[file_idx];
                    work_item._file._path = work_item._path.clone();
                    work_item._file._size = file._length as u64;
                    work_item._file._position = 0;
                    work_item._file._device = Some(this.clone());
                    work_item._file._handle = Some(self.decompress_as_handle(file));
                    work_item._status = kStatusComplete;
                } else {
                    work_item._status = kStatusInvalidPath
                }
            }
            BuildTool::TfbTool => {
                let file_name = &work_item._path[(work_item._path.rfind('/').unwrap() + 1)..];
                let archive_path = &work_item._path[..work_item._path.rfind('/').unwrap()];
                
                if archive_path == self._path {
                    for file in &self._files {
                        if file._name == file_name {
                            work_item._file._path = work_item._path.clone();
                            work_item._file._size = file._length as u64;
                            work_item._file._position = 0;
                            work_item._file._device = Some(this.clone());
                            work_item._file._handle = Some(self.decompress_as_handle(file));
                            work_item._status = kStatusComplete;
                            return
                        }
                    }
                }
            }
            _ => panic!("Unsupported Game Tooling"),
        }
    }

    fn close(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        work_item._status = kStatusUnsupported
    }

    fn read(&self, _this: Arc<Mutex<dyn igFileWorkItemProcessor>>, work_item: &mut igFileWorkItem) {
        work_item._status = kStatusUnsupported
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
        work_item._status = kStatusUnsupported
    }

    fn get_file_list(
        &self,
        _this: Arc<Mutex<dyn igFileWorkItemProcessor>>,
        work_item: &mut igFileWorkItem,
    ) {
        match &mut work_item._buffer {
            WorkItemBuffer::StringRefList(files) => {
                for file_info in &self._files {
                    files.push(file_info._logical_name.clone())
                }
            }
            _ => {
                work_item._status = kStatusGeneralError;
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
        if delete(&work_item._path).is_ok() {
            work_item._status = kStatusComplete
        } else {
            work_item._status = kStatusInvalidPath
        }
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

pub struct Header {
    /// Custom field added by ig-workshop. Not present in real igArchives
    pub endian: Endian,
    pub _magic_number: u32,
    pub _version: u32,
    pub _toc_size: u32,
    pub _num_files: u32,
    pub _sector_size: u32,
    pub _hash_search_divider: u32,
    pub _hash_search_slop: u32,
    pub _num_large_file_blocks: u32,
    pub _num_medium_file_blocks: u32,
    pub _num_small_file_blocks: u32,
    pub _name_table_offset: u64,
    pub _name_table_size: u32,
    pub _flags: u32,
}

/// <summary>
/// Different compression formats
/// </summary>
#[derive(Debug)]
#[repr(usize)]
pub enum CompressionType {
    kUncompressed = 0,
    kZlib = 1,
    kLzma = 2,
    kLz4 = 3,
    kCompressionFormatShift = 28,
    kCompressionFormatMask = 0xF0000000,
    kFirstBlockMask = 0x0FFFFFFF,
    kOffsetBits = 40,
}

impl CompressionType {
    fn from_index(block_index: u32) -> CompressionType {
        let shift = block_index >> 28;

        match shift {
            0 => CompressionType::kUncompressed,
            1 => CompressionType::kZlib,
            2 => CompressionType::kLzma,
            3 => CompressionType::kLz4,
            28 => CompressionType::kCompressionFormatShift,
            0xF0000000 => CompressionType::kCompressionFormatMask,
            0x0FFFFFFF => CompressionType::kFirstBlockMask,
            40 => CompressionType::kOffsetBits,
            _ => panic!("Unknown compression type"),
        }
    }
}

/// <summary>
/// Different block sizes
/// </summary>
pub enum EBlockType {
    kSmall,
    kMedium,
    kLarge,
    kNone,
}

pub struct FileInfo {
    /// The hash of the file
    pub _offset: u32,
    /// The offset within the file (do we really need to store this?)
    pub _ordinal: u32,
    /// The ordinal, this represents the order of how the compressed data is written (could this be inferred?)
    pub _length: u32,
    /// The uncompressed length of the file
    pub _block_index: u32,
    /// Change this for just compression mode at some point, the block index is useless now
    pub _name: String,
    /// The "real" name of the file
    pub _logical_name: String,
    /// The "logical" name of the file, used to compute its hash
    pub _modification_time: u32,
    /// The modification time of the file (for some reason this is never accurate)
    pub _blocks: Option<Vec<u32>>,
    /// The block information
    pub _compressed_data: Vec<u8>,
    /// The actual compressed data
    pub _hash: u32,
}

impl FileInfo {
    pub fn get_block_type(&self, sector_size: u32) -> EBlockType {
        if self._blocks.is_none() {
            return EBlockType::kNone;
        }

        if 0x7F * sector_size < self._length {
            if 0x7FFF * sector_size < self._length {
                return EBlockType::kLarge;
            }

            return EBlockType::kMedium;
        }

        EBlockType::kSmall
    }
}
