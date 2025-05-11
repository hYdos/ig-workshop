use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use std::any::TypeId;
use std::io::Cursor;

pub struct igSizeTypeMetaField;

impl igMetaField for igSizeTypeMetaField {
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn value_from_igz(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &IgzLoaderContext,
        _registry: &igMetafieldRegistry,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igz(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgzSaverContext,
    ) -> Result<(), IgzSaverError> {
        todo!()
    }

    fn value_from_igx(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgxLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igx(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgxSaverContext,
    ) -> Result<(), IgxSaverError> {
        todo!()
    }

    fn value_from_igb(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgbLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igb(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError> {
        todo!()
    }

    fn platform_size(&self, _platform: IG_CORE_PLATFORM) -> u32 {
        todo!()
    }

    fn platform_alignment(&self, _platform: IG_CORE_PLATFORM) -> u32 {
        todo!()
    }
}
