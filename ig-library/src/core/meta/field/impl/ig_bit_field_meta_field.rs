use crate::core::ig_fs::Endian;
use crate::core::ig_objects::{igAny, igObjectStreamManager};
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::core::meta::ig_xml_metadata::BitShiftInfo;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use std::any::{Any, TypeId};
use std::io::Cursor;
use std::sync::{Arc, RwLock};

pub(crate) struct igBitFieldMetaField(pub Arc<RwLock<BitShiftInfo>>, pub Arc<dyn igMetaField>);

impl igMetaField for igBitFieldMetaField {
    fn type_of(&self) -> TypeId {
        self.1.type_of()
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
        let bit_info = self.0.read().unwrap();
        let raw_storage = self.1.value_from_igz(
            registry,
            metadata_manager,
            object_stream_manager,
            handle,
            endian.clone(),
            ctx,
        )?;

        let mut guard = raw_storage.write().unwrap();
        let mut storage = match self.1.type_of() {
            t if t == TypeId::of::<u8>() => *guard.downcast_mut::<u8>().unwrap() as u64,
            t if t == TypeId::of::<u16>() => *guard.downcast_mut::<u16>().unwrap() as u64,
            t if t == TypeId::of::<u32>() => *guard.downcast_mut::<u32>().unwrap() as u64,
            t if t == TypeId::of::<u64>() => *guard.downcast_mut::<u64>().unwrap(),
            t if t == TypeId::of::<i8>() => *guard.downcast_mut::<i8>().unwrap() as u64,
            t if t == TypeId::of::<i16>() => *guard.downcast_mut::<i16>().unwrap() as u64,
            t if t == TypeId::of::<i32>() => *guard.downcast_mut::<i32>().unwrap() as u64,
            t if t == TypeId::of::<i64>() => *guard.downcast_mut::<i64>().unwrap() as u64,
            _ => {
                todo!("Unable to decode storage type. contact hydos")
            }
        };
        drop(guard);
        storage = (storage >> bit_info.shift) & (u64::MAX >> (64 - bit_info.bits));

        match self.type_of() {
            t if t == TypeId::of::<bool>() => Some(Arc::new(RwLock::new(storage != 0))),
            t if t == TypeId::of::<u8>() => Some(Arc::new(RwLock::new(storage as u8))),
            t if t == TypeId::of::<u16>() => Some(Arc::new(RwLock::new(storage as u16))),
            t if t == TypeId::of::<u32>() => Some(Arc::new(RwLock::new(storage as u32))),
            t if t == TypeId::of::<u64>() => Some(Arc::new(RwLock::new(storage))),
            t if t == TypeId::of::<i8>() => Some(Arc::new(RwLock::new(storage as i8))),
            t if t == TypeId::of::<i16>() => Some(Arc::new(RwLock::new(storage as i16))),
            t if t == TypeId::of::<i32>() => Some(Arc::new(RwLock::new(storage as i32))),
            t if t == TypeId::of::<i64>() => Some(Arc::new(RwLock::new(storage as i64))),
            _ => {
                todo!("Missing type handling for igBitFieldMetaField")
            }
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
