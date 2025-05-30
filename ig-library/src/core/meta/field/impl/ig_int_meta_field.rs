use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use crate::util::byteorder_fixes::read_i32;
use std::any::TypeId;
use std::io::Cursor;
use std::sync::{Arc, RwLock};

pub(crate) struct igIntMetaField;

impl igMetaField for igIntMetaField {
    fn type_id(&self) -> TypeId {
        TypeId::of::<i32>()
    }

    fn value_from_igz(&self, handle: &mut Cursor<Vec<u8>>, endian: &Endian, ctx: &IgzLoaderContext, registry: &igMetafieldRegistry, metadata_manager: &igMetadataManager) -> Option<igAny> {
        Some(Arc::new(RwLock::new(read_i32(handle, endian).unwrap())))
    }

    fn value_into_igz(&self, handle: &mut Cursor<Vec<u8>>, endian: &Endian, ctx: &mut IgzSaverContext) -> Result<(), IgzSaverError> {
        todo!()
    }

    fn value_from_igx(&self, handle: &mut Cursor<Vec<u8>>, endian: &Endian, ctx: &mut IgxLoaderContext) -> Option<igAny> {
        todo!()
    }

    fn value_into_igx(&self, handle: &mut Cursor<Vec<u8>>, endian: &Endian, ctx: &mut IgxSaverContext) -> Result<(), IgxSaverError> {
        todo!()
    }

    fn value_from_igb(&self, handle: &mut Cursor<Vec<u8>>, endian: &Endian, ctx: &mut IgbLoaderContext) -> Option<igAny> {
        todo!()
    }

    fn value_into_igb(&self, handle: &mut Cursor<Vec<u8>>, endian: &Endian, ctx: &mut IgbSaverContext) -> Result<(), IgbSaverError> {
        todo!()
    }

    fn platform_size(&self, ig_metadata_manager: &igMetadataManager, platform: IG_CORE_PLATFORM) -> u32 {
        4
    }

    fn platform_alignment(&self, ig_metadata_manager: &igMetadataManager, platform: IG_CORE_PLATFORM) -> u32 {
        4
    }
}