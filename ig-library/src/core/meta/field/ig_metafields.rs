use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use std::io::Cursor;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::ig_metadata_manager::igMetadataManager;

/// Implementation of a meta field that allows for serialization/deserialization of the type.
pub trait igMetaField: Send + Sync {
    fn type_id(&self) -> std::any::TypeId;

    /// Takes a value in an igz and will convert it into <T>. Will return [None] when the read value is "null"
    fn value_from_igz(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        ctx: &IgzLoaderContext,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
    ) -> Option<igAny>;
    /// Accepts a value of type <T> and will return [Ok] if successful. If an error occurred, the type [IgzSaverError] will be returned hopefully containing useful information for debugging
    fn value_into_igz(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        ctx: &mut IgzSaverContext,
    ) -> Result<(), IgzSaverError>;

    /// Takes a value in an igx and will convert it into <T>. Will return [None] when the read value is "null"
    fn value_from_igx(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        ctx: &mut IgxLoaderContext,
    ) -> Option<igAny>;
    /// Accepts a value of type <T> and will return [Ok] if successful. If an error occurred, the type [IgxSaverError] will be returned hopefully containing useful information for debugging
    fn value_into_igx(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        ctx: &mut IgxSaverContext,
    ) -> Result<(), IgxSaverError>;

    /// Takes a value in an igb and will convert it into <T>. Will return [None] when the read value is "null"
    fn value_from_igb(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        ctx: &mut IgbLoaderContext,
    ) -> Option<igAny>;
    /// Accepts a value of type <T> and will return [Ok] if successful. If an error occurred, the type [IgbSaverError] will be returned hopefully containing useful information for debugging
    fn value_into_igb(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: &Endian,
        ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError>;

    /// The size of the value on the specified platform
    fn platform_size(&self, ig_metadata_manager: &igMetadataManager, platform: IG_CORE_PLATFORM) -> u32;

    /// The alignment needed to be used on the specified platform
    fn platform_alignment(&self, ig_metadata_manager: &igMetadataManager, platform: IG_CORE_PLATFORM) -> u32;
}
