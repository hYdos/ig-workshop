use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_memory::igMemoryPool;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::ig_metadata_manager::igMetadataManager;

#[allow(dead_code)] // TODO: i need to look into this more. it seems cauldron's implementation (what this is based on) isn't completely finished and leaves a few things out... Will check in ghidra later hopefully
pub struct igMemory<T> where T: 'static + Send + Sync {
    pub data: Vec<T>,
    pub pool: igMemoryPool,
    pub implicit_memory_pool: bool,
    pub optimal_cpuread_write: bool,
    pub optimal_gpuread: bool,
    pub alignment_multiple: u32
}

impl<T> igMemory<T>
where
    T: 'static + Send + Sync
{
    /// Takes the flags specified and writes their values into [Self]
    pub fn set_flags(&mut self, metadata_manager: &igMetadataManager, flags: u64, meta_field: &dyn igMetaField, platform: IG_CORE_PLATFORM) {
        let alignment: u32;
        let size: u64;
        
        if platform.is_64bit() {
            alignment = 1 << ((((flags >> 0x3B) & 0xF) + 2) as u32);
            self.optimal_cpuread_write = (flags >> 0x3F) != 0;
            size = flags & 0x07FF_FFFF_FFFF_FFFF;
        } else {
            alignment = 1 << ((((flags >> 0x1B) & 0xF) + 2) as u32);
            self.optimal_cpuread_write = (flags >> 0x1F) != 0;
            size = flags & 0x07FF_FFFF;
        }
        
        self.alignment_multiple = alignment / meta_field.platform_alignment(metadata_manager, platform.clone());
        self.data = Vec::with_capacity((size / meta_field.platform_size(metadata_manager, platform) as u64) as usize)
    }
}

impl<T> igMemory<T>
where
    T: 'static + Send + Sync
{
    pub(crate) fn new() -> Self {
        igMemory {
            data: vec![],
            pool: igMemoryPool::Default,
            implicit_memory_pool: false,
            optimal_cpuread_write: false,
            optimal_gpuread: false,
            alignment_multiple: 0,
        }
    }
}