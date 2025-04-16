use crate::core::fs::igFileDescriptor;
use crate::core::ig_file_context::igFileContext;

/// Represents an archive file
pub struct igArchive {
    pub _load_name_table: bool,
    pub _sequential_read: bool,
    pub _loading_for_incremental_update: bool,
    pub _enable_cache: bool,
    pub _override: bool,
    pub _file_descriptor: igFileDescriptor,
    pub _open: bool,
    pub _configured: bool,
    pub _needs_endian_swap: bool,
    pub _archive_header: Header,
    pub _files: Vec<FileInfo>,
    pub _native_media: String,
    pub _native_path: String,
    pub _native_app_path: String,
}

impl igArchive {

    /// Opens an archive
    /// file_path is the path of the archive
    pub fn open(file_context: &igFileContext, file_path: String) -> Result<igArchive, &'static str> {
        let file_descriptor = file_context.open(file_path, 0);
        
        Err("todo igArchive::open")
    }
    
    pub fn new() -> Self {
        igArchive {
            _load_name_table: false,
            _sequential_read: false,
            _loading_for_incremental_update: false,
            _enable_cache: false,
            _override: false,
            _file_descriptor: igFileDescriptor::empty(),
            _open: false,
            _configured: false,
            _needs_endian_swap: false,
            _archive_header: Header {
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

pub struct Header {
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
    pub _name_table_offset: u32,
    pub _name_table_size: u32,
    pub _flags: u32,
}

/// <summary>
/// Different compression formats
/// </summary>
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
