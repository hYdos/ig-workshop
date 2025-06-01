use crate::core::ig_fs::Endian;
use crate::core::ig_objects::{igAny, igObjectStreamManager};
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use std::io::Cursor;

/// Implementation of a meta field that allows for serialization/deserialization of the type.
pub trait igMetaField: Send + Sync {
    fn type_id(&self) -> std::any::TypeId;

    /// Takes a value in an igz and will convert it into <T>. Will return [None] when the read value is "null"
    fn value_from_igz(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgzLoaderContext,
    ) -> Option<igAny>;
    /// Accepts a value of type <T> and will return [Ok] if successful. If an error occurred, the type [IgzSaverError] will be returned hopefully containing useful information for debugging
    fn value_into_igz(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgzSaverContext,
    ) -> Result<(), IgzSaverError>;

    /// Takes a value in an igx and will convert it into <T>. Will return [None] when the read value is "null"
    fn value_from_igx(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgxLoaderContext,
    ) -> Option<igAny>;
    /// Accepts a value of type <T> and will return [Ok] if successful. If an error occurred, the type [IgxSaverError] will be returned hopefully containing useful information for debugging
    fn value_into_igx(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgxSaverContext,
    ) -> Result<(), IgxSaverError>;

    /// Takes a value in an igb and will convert it into <T>. Will return [None] when the read value is "null"
    fn value_from_igb(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgbLoaderContext,
    ) -> Option<igAny>;
    /// Accepts a value of type <T> and will return [Ok] if successful. If an error occurred, the type [IgbSaverError] will be returned hopefully containing useful information for debugging
    fn value_into_igb(
        &self,
        registry: &igMetafieldRegistry,
        metadata_manager: &igMetadataManager,
        object_stream_manager: &igObjectStreamManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError>;
}
