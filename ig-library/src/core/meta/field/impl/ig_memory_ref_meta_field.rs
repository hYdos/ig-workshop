use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::memory::igMemory;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::ig_metadata_manager::{igMetaFieldInfo, igMetadataManager};
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use crate::util::byteorder_fixes::{read_ptr, read_struct_array_u8_ref};
use std::any::TypeId;
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use log::debug;

pub(crate) struct igMemoryRefMetaField(pub Arc<igMetaFieldInfo>);

impl igMetaField for igMemoryRefMetaField {
    fn type_id(&self) -> TypeId {
        TypeId::of::<igMemory<igAny>>()
    }

    fn value_from_igz(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &IgzLoaderContext,
    ) -> Option<igAny> {
        debug!("Internal meta object type={}", self.0._type);
        let start = handle.position();
        let flags = read_ptr(handle, ctx.platform.clone(), endian.clone()).unwrap();
        let raw = read_ptr(handle, ctx.platform.clone(), endian.clone()).unwrap();

        let offset = ctx.deserialize_offset(raw);
        let mut memory: igMemory<igAny> = igMemory::new(); // We don't know the type inside the memory, we didn't create it. However, we know the metafield so we know what is supposed to be here, making it safe in the end.

        // TODO: make 2 constructors for igMemory: one takes a pool and the other a set of flags. This fits in with rust's structuring where nothing should be used until initialized and guarantees better safety
        if ctx.runtime_fields.pool_ids.binary_search(&start).is_ok() {
            memory.pool = ctx.loaded_pools[(flags & 0xFFFFFF) as usize];
        } else {
            memory.set_flags(flags, ctx.platform.get_pointer_size(), ctx.platform.get_pointer_size() * 2, ctx.platform.clone());
            memory.pool = ctx.get_pool_from_serialized_offset(raw);

            let guard = self.0.ark_info.read().unwrap();
            // Optimized u8 slice copy
            if guard._type.as_ref() == "igUnsignedCharMetaField" {
                handle.set_position(offset);
                let slice = read_struct_array_u8_ref(handle, endian.clone(), memory.data.len()).unwrap();
                for x in slice {
                    handle.set_position(offset);
                    memory.data.push(Arc::new(RwLock::new(x)));
                }
            } else {
                let inner_meta_field = registry.get_simple(&self.0.ark_info.read().unwrap());
                for i in 0..memory.data.capacity() {
                    handle.set_position(offset + (self.0.size as u64) * (i as u64));
                    memory.data.push(inner_meta_field.value_from_igz(
                        registry,
                        metadata_manager,
                        handle,
                        endian.clone(),
                        ctx,
                    )?)
                }
            }
        }

        Some(Arc::new(RwLock::new(memory)))
    }

    fn value_into_igz(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
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
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError> {
        todo!()
    }
}
