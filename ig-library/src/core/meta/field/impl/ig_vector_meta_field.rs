use crate::core::ig_fs::Endian;
use crate::core::ig_objects::{igAny, igObjectStreamManager};
use crate::core::ig_vector::igVector;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::memory::igMemory;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::field::r#impl::ig_memory_ref_meta_field::igMemoryRefMetaField;
use crate::core::meta::ig_metadata_manager::{igMetaFieldInfo, igMetadataManager};
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use crate::util::byteorder_fixes::{read_u32, read_u64};
use std::any::{Any, TypeId};
use std::io::Cursor;
use std::sync::{Arc, RwLock};

pub(crate) struct igVectorMetaField(pub Arc<igMetaFieldInfo>);

impl igMetaField for igVectorMetaField {
    fn type_of(&self) -> TypeId {
        TypeId::of::<igVector<igAny>>()
    }

    fn value_from_igz(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgzLoaderContext,
    ) -> Option<igAny> {
        let count = if ctx.version == 0x09 && ctx.platform.is_64bit() {
            read_u64(handle, endian.clone()).unwrap()
        } else {
            read_u32(handle, endian.clone()).unwrap() as u64
        };

        if count > 0 {
            let memory_reader = igMemoryRefMetaField(self.0.clone());

            let ig_memory = memory_reader
                .value_from_igz(
                    _registry,
                    _metadata_manager,
                    _object_stream_manager,
                    handle,
                    endian,
                    ctx,
                )
                .unwrap();

            // This line validates later usage of unsafe. Makes sure the type we are casting to is actually that type.
            assert!(ig_memory.read().ok()?.is::<igMemory<igAny>>());
            let raw: *const RwLock<dyn Any + Send + Sync> = Arc::into_raw(ig_memory);
            let raw_t: *const RwLock<igMemory<igAny>> = raw.cast();
            let typed: Arc<RwLock<igMemory<igAny>>> = unsafe { Arc::from_raw(raw_t) };

            Some(Arc::new(RwLock::new(igVector::from_memory(typed, count))))
        } else {
            Some(Arc::new(RwLock::new(igVector::<igAny>::new())))
        }
    }

    fn value_into_igz(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _object_stream_manager: &igObjectStreamManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgzSaverContext,
    ) -> Result<(), IgzSaverError> {
        todo!()
    }

    fn value_from_igx(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _object_stream_manager: &igObjectStreamManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgxLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igx(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _object_stream_manager: &igObjectStreamManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgxSaverContext,
    ) -> Result<(), IgxSaverError> {
        todo!()
    }

    fn value_from_igb(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _object_stream_manager: &igObjectStreamManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgbLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igb(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _object_stream_manager: &igObjectStreamManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError> {
        todo!()
    }
}
