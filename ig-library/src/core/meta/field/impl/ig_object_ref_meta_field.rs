use crate::core::ig_fs::Endian;
use crate::core::ig_objects::{igAny, igObject, igObjectStreamManager};
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use std::any::TypeId;
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use log::error;
use crate::core::meta::field::r#impl::ig_size_type_meta_field::igSizeTypeMetaField;

pub struct igObjectRefMetaField;

impl igMetaField for igObjectRefMetaField {
    fn type_id(&self) -> TypeId {
        TypeId::of::<igObject>()
    }

    fn value_from_igz(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgzLoaderContext,
    ) -> Option<igAny> {
        let base_offset = handle.position();
        let size_type_meta_field = igSizeTypeMetaField;
        let raw = *size_type_meta_field.value_from_igz(
            registry,
            metadata_manager,
            object_stream_manager,
            handle,
            endian,
            ctx,
        ).unwrap().read().unwrap().downcast_ref::<u64>().unwrap();

        let is_offset = ctx.runtime_fields.offsets.binary_search(&base_offset).is_ok();
        if is_offset {
            return Some(Arc::new(RwLock::new(ctx.offset_object_list[&raw].clone())));
        }
        let is_named_external = ctx.runtime_fields.named_externals.binary_search(&base_offset).is_ok();
        if is_named_external {
            return Some(Arc::new(RwLock::new(ctx.named_external_list[(raw & 0x7FFFFFFF) as usize].clone())));
        }
        let is_exid = ctx.runtime_fields.externals.binary_search(&base_offset).is_ok();
        if is_exid {
            return if let Some(obj) = ctx.external_list[(raw & 0x7FFFFFFF) as usize].get_object_alias(object_stream_manager) {
                Some(Arc::new(RwLock::new(obj)))
            } else {
                None
            };
        }
        if raw != 0 {
            // value should not be null, but we couldn't determine what it actually was.
            error!("Failed to read igObjectRefMetaField properly");
            panic!("Alchemy Error! Check the logs.");
        }

        None
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
